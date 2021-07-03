pub mod user;

mod dto;

#[get("/hello")]
pub fn index() -> &'static str {
    "Hello, world!"
}