use rocket::get;
use rocket::Request;

#[get("/")]
pub fn index() -> &'static str{
    "Hello-O-aza"
}

#[get("/hello")]
pub fn hello() -> &'static str{
    "hello"
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("404 {}",req.uri())
}