#![feature(proc_macro_hygiene,decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str{
    "Hello-O-aza!"
}

#[get("/hello")]
fn hello() -> &'static str{
    "hello"
}

fn main(){
    rocket::ignite().mount("/",routes![index,hello]).launch();
}