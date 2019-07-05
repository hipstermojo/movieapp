#[macro_use]
extern crate log;

use std::env;
use std::io;

use actix_web::{web, App, HttpServer};
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
mod handler;
mod model;


fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "movieapp=debug,actix_web=debug");
    env_logger::init();

    info!("Connecting to database");

    let db_host = env::var("MONGO_HOST").expect("MONGO_HOST variable must be set");
    let db_port = env::var("MONGO_PORT").expect("MONGO_PORT variable must be set");
    let db_port: u16 = db_port.parse().expect("Invalid port number in MONGO_PORT");
    let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME variable must be set");

    let db_conn_manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(&db_host, db_port)
            .with_db(&db_name)
            .build(),
    );

    let pool = Pool::builder()
        .max_size(16)
        .build(db_conn_manager)
        .expect("Unable to build connection pool");

    info!("Database connection successful");

    let host = env::var("HOST").expect("HOST variable must be set");
    let port = env::var("PORT").expect("PORT variable must be set");
    let ip_addr = host.to_owned() + ":" + &port;

    info!("Starting Actix-Web server");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/").to(|| "Hello World"))
    })
    .bind(&ip_addr)?
    .run()
}
