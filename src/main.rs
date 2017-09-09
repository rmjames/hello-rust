#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("../public/index.html")
}

#[get("/about")]
fn about() -> &'static str {
    "About"
}

fn main() {
    rocket::ignite().mount("/", routes![index, about]).launch();
}


// #![feature(plugin)]
// #![plugin(rocket_codegen)]

// extern crate rocket;

// #[get("/hello/<name>/<age>")]
// fn hello(name: String, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }

// fn main() {
//     rocket::ignite().mount("/", routes![hello]).launch();
//}