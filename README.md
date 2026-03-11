# diesel-enum-number

Maps Rust enums with explicit integer discriminants to PostgreSQL `SMALLINT` columns via diesel.
This code was adapted from a declarative macro that worked the same way but was not available an attribute macro,
which became a bit of a pain when leveraging on type generation via [typeshare](https://github.com/1Password/typeshare)
due to limitations with attaching the attribute to the generated enum.

## Usage

```rust
use diesel_enum_number::diesel_enum_number;

#[diesel_enum_number]
// serde not required, but can fit in well with this approach
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    Active = 1,
    Inactive = 2,
    Pending = 3,
}

/// expands to:
/// #[repr(i16)]
/// #[derive(diesel::AsExpression, diesel::FromSqlRow)]
/// #[diesel(sql_type = diesel::sql_types::SmallInt)]
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
/// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// pub enum UserStatus {
///     Active = 1,
///     Inactive = 2,
///     Pending = 3,
/// }
/// // + ToSql, FromSql, and from_i16 impls
```

The enum can then be used directly in diesel structs and queries:

```rust
#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub status: UserStatus,
}

users::table
.filter(users::status.eq(UserStatus::Active))
.load::<User>(conn) ?;
```

## vs. diesel-derive-enum

[diesel-derive-enum](https://crates.io/crates/diesel-derive-enum) maps Rust enums to PostgreSQL native enum types (
`CREATE TYPE status AS ENUM ('active', 'inactive')`), which are string-based.
This crate instead supports using small integer-based enum approach, deferring the meaning of the values to the
application layer as well as avoiding migrations when making changes to existing enums etc.

## Backend support

Currently only PostgreSQL is supported. Support for MySQL and SQLite may be added in a future release.

## License

MIT OR Apache-2.0
