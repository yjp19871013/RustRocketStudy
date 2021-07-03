use diesel::mysql::MysqlConnection;
use crate::db::user;

pub fn create_user<'a>(conn: &mut MysqlConnection, user_name: &'a str, password: &'a str) -> Result<(), String> {
    if user_name == "" || password == "" {
        return Err(String::from("没有传递必要的参数"))
    }

    match user::create_user(conn, user_name, password) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("创建用户失败: {}", e))
    }
}