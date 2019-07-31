#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tera;

#[macro_use]
extern crate mongodb;

use std::env;
use std::io;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::client::Client;
use actix_web::{middleware, web, App, HttpServer};
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use tera::Tera;

mod handler;
mod model;
mod utils;

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
        .max_size(5)
        .build(db_conn_manager)
        .expect("Unable to build connection pool");

    info!("Database connection successful");

    let host = env::var("HOST").expect("HOST variable must be set");
    let port = env::var("PORT").expect("PORT variable must be set");
    let ip_addr = host.to_owned() + ":" + &port;

    let api_key: model::APIKey =
        env::var("TMDB_API_KEY").expect("TMDB_API_KEY variable must be set");

    info!("Starting Actix-Web server");

    HttpServer::new(move || {
        let templates: Tera = compile_templates!("templates/**/*");
        App::new()
            .data(templates)
            .data(Client::new())
            .data(pool.clone())
            .data(api_key.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("actix-auth")
                    .secure(false),
            ))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to_async(handler::fetch_movies_now_playing))
            .service(
                web::resource("/login")
                    .route(web::get().to(handler::login_view))
                    .route(web::post().to(handler::login_handler)),
            )
            .service(
                web::resource("/signup")
                    .route(web::get().to(handler::signup_view))
                    .route(web::post().to_async(handler::new_user_handler)),
            )
            .service(fs::Files::new("/static", "static/"))
    })
    .bind(&ip_addr)?
    .run()
}
