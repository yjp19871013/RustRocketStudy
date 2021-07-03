use diesel::mysql::MysqlConnection;
use super::schema;
use diesel::prelude::*;
use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    user_name: &'a str,
    password : &'a str,
}

pub fn create_user<'a>(conn: &mut MysqlConnection, user_name: &'a str, password: &'a str) 
    -> Result<usize, diesel::result::Error> {
    let new_user = NewUser {
        user_name: user_name,
        password: password,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)
}