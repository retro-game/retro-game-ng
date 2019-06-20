#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;
use std::fs::read_to_string;

mod controller;
mod model;
mod schema;
mod service;
mod util;
mod validation;
mod view;

#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    address: String,
    port: u16,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    url: String,
    pool_size: Option<u32>,
}

static CONFIG_PATH: &str = "config.toml";

fn load_config() -> Config {
    let content = read_to_string(CONFIG_PATH).unwrap();
    toml::from_str(&content).unwrap()
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppData {
    pub db_pool: DbPool,
}

fn create_db_pool(url: &str, size: u32) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(size)
        .build(manager)
        .expect("Failed to create db pool")
}

fn main() -> std::io::Result<()> {
    let config = load_config();

    println!("Creating db pool");
    let size = config.database.pool_size.unwrap_or(1);
    let db_pool = create_db_pool(&config.database.url, size);

    println!("Creating server");
    HttpServer::new(move || {
        App::new()
            .data(AppData {
                db_pool: db_pool.clone(),
            })
            .service(controller::home::get)
            .service(controller::join::get)
            .service(controller::join::post)
            .service(controller::overview::get)
            .service(controller::sign_in::post)
    })
    .bind(format!("{}:{}", config.server.address, config.server.port))?
    .run()
}
