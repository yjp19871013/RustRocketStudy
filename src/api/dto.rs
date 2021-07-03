use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MsgResponse {
    pub success: bool,
    pub msg: String
}

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user_name: String,
    pub password: String
}