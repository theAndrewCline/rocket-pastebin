#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket::http::RawStr;
use rocket::{get, routes, Data};
use std::io;
use std::path::Path;
mod paste_id;
use paste_id::PasteId;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
