#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

pub mod db;
pub mod service;
pub mod api;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("migrations");

    let conn = db::DBConn::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}

#[rocket::main]
async fn main() {
    match rocket::build()
        .attach(db::DBConn::fairing())
        .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
        .mount("/", routes![api::index])
        .mount("/", routes![api::user::create_user])
        .launch().await {
            Err(e) => {println!("Shutdown server error: {}", e)},
            Ok(_) => {}
        };
}