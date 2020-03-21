#[macro_use]
extern crate tower_web;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv_codegen;

extern crate dotenv;
extern crate serde;
extern crate serde_json;

use tower_web::middleware::log::LogMiddleware;
use tower_web::ServiceBuilder;

pub mod db;
pub mod handlers;
pub mod model;
pub mod schema;

pub fn main() {
    let pool = db::connect();
    let addr = "127.0.0.1:8088".parse().expect("Invalid address");

    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(handlers::users::UsersAPI::new(pool))
        //.middleware(LogMiddleware::new(module_path!()))
        .run(&addr)
        .unwrap();
}
