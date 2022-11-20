use crate::context::Context;
use std::path::Path;
use std::thread;

use rocket::fs::NamedFile;
use rocket::{get, routes, State};

// #[get("/world")]
// fn world() -> &'static str {
//     "Hello, world!"
// }

#[get("/proxy.js")]
async fn proxy() -> Option<NamedFile> {
    NamedFile::open("/Users/larry/.v2up/pac.js").await.ok()
    // String::from("hello world")
}

#[rocket::main]
pub async fn run() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/pac", routes![proxy]).launch().await?;
    Ok(())
}
