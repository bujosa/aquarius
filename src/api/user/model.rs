use poem_openapi::{
    types::{Email, Password},
    Object,
};

/// Create user schema
#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct User {
    /// Id
    #[oai(read_only)]
    pub id: i64,
    /// Name
    #[oai(validator(max_length = 64))]
    pub name: String,
    /// Password
    #[oai(validator(max_length = 32))]
    pub password: Password,
    pub email: Email,
}
