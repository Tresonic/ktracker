use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct IdUser {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub hash: String,
    pub salt: String,
}