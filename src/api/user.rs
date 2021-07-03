use crate::service;
use super::dto;
use crate::db;
use rocket::serde::json::Json;

#[post("/user", data = "<user>")]
pub async fn create_user(conn: db::DBConn, user: Json<dto::User>) -> Json<dto::MsgResponse> {
    match conn.run(move |c| {
        service::user::create_user(c, &user.user_name, &user.password)
    }).await {
        Ok(_) => {
            return Json(dto::MsgResponse {
                success: true,
                msg: String::new()
            })
        },
        Err(e) => {
            return Json(dto::MsgResponse {
                success: false,
                msg: e
            })
        }
    }
}
