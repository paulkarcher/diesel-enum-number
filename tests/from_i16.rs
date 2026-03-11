use diesel_enum_number::diesel_enum_number;

#[diesel_enum_number]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active = 1,
    Inactive = 2,
    Pending = 3,
}

#[diesel_enum_number]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Offset {
    Negative = -1,
    Zero = 0,
    Positive = 1,
}

#[test]
fn valid_values_roundtrip() {
    assert_eq!(Status::from_i16(1), Ok(Status::Active));
    assert_eq!(Status::from_i16(2), Ok(Status::Inactive));
    assert_eq!(Status::from_i16(3), Ok(Status::Pending));
}

#[test]
fn invalid_value_returns_error() {
    assert!(Status::from_i16(0).is_err());
    assert!(Status::from_i16(99).is_err());
    assert!(Status::from_i16(-1).is_err());
}

#[test]
fn error_message_includes_value_and_type_name() {
    let err = Status::from_i16(42).unwrap_err();
    assert!(err.contains("42"));
    assert!(err.contains("Status"));
}

#[test]
fn negative_discriminants_roundtrip() {
    assert_eq!(Offset::from_i16(-1), Ok(Offset::Negative));
    assert_eq!(Offset::from_i16(0), Ok(Offset::Zero));
    assert_eq!(Offset::from_i16(1), Ok(Offset::Positive));
}
