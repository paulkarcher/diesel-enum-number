use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemEnum, parse_macro_input};

/// Attribute macro that implements `from_i16`, `ToSql<SmallInt, Pg>`, and
/// `FromSql<SmallInt, Pg>` for an enum, and automatically applies `#[repr(i16)]`.
///
/// All variants must be unit variants with explicit discriminants.
///
/// # Example
/// ```ignore
/// use diesel_enum_number::diesel_enum_number;
///
/// #[diesel_enum_number]
/// // serde not required, but can fit in well with this approach
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
/// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// pub enum UserStatus {
///     Active = 1,
///     Inactive = 2,
/// }
/// ```
///
/// expands to:
///
/// ```ignore
/// #[repr(i16)]
/// #[derive(diesel::AsExpression, diesel::FromSqlRow)]
/// #[diesel(sql_type = diesel::sql_types::SmallInt)]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
/// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// pub enum UserStatus {
///     Active = 1,
///     Inactive = 2,
/// }
/// // + ToSql, FromSql, and from_i16 impls
/// ```
#[proc_macro_attribute]
pub fn diesel_enum_number(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let name = &input.ident;

    let match_arms: Vec<_> = input
        .variants
        .iter()
        .map(|variant| {
            assert!(
                matches!(variant.fields, Fields::Unit),
                "diesel_enum_number requires unit variants, but `{}::{}` has fields",
                name,
                variant.ident
            );
            let ident = &variant.ident;
            let discriminant = variant
                .discriminant
                .as_ref()
                .map(|(_, expr)| expr)
                .unwrap_or_else(|| {
                    panic!(
                        "Variant `{}::{}` must have an explicit discriminant",
                        name, ident
                    )
                });
            quote! { #discriminant => Ok(#name::#ident), }
        })
        .collect();

    let expanded = quote! {
        #[repr(i16)]
        #[derive(::diesel::AsExpression, ::diesel::FromSqlRow)]
        #[diesel(sql_type = ::diesel::sql_types::SmallInt)]
        #input

        impl #name {
            pub fn from_i16(value: i16) -> Result<Self, String> {
                match value {
                    #(#match_arms)*
                    _ => Err(format!("Invalid value {} for {}", value, stringify!(#name))),
                }
            }
        }

        impl ::diesel::serialize::ToSql<::diesel::sql_types::SmallInt, ::diesel::pg::Pg> for #name {
            fn to_sql<'b>(
                &'b self,
                out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::pg::Pg>,
            ) -> ::diesel::serialize::Result {
                let value = *self as i16;
                ::diesel::serialize::ToSql::<::diesel::sql_types::SmallInt, ::diesel::pg::Pg>::to_sql(
                    &value,
                    &mut out.reborrow(),
                )
            }
        }

        impl ::diesel::deserialize::FromSql<::diesel::sql_types::SmallInt, ::diesel::pg::Pg> for #name {
            fn from_sql(
                bytes: ::diesel::pg::PgValue<'_>,
            ) -> ::diesel::deserialize::Result<Self> {
                let value = <i16 as ::diesel::deserialize::FromSql<
                    ::diesel::sql_types::SmallInt,
                    ::diesel::pg::Pg,
                >>::from_sql(bytes)?;
                #name::from_i16(value).map_err(|e| e.into())
            }
        }
    };

    TokenStream::from(expanded)
}
