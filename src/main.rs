#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

use actix_session::CookieSession;
use actix_web::cookie::SameSite;
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use log::info;

mod config;
mod context;
mod controller;
mod db;
mod model;
mod schema;
mod service;
mod util;
mod validation;
mod view;

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

fn make_session_middleware() -> CookieSession {
    // FIXME: Load the key from config. Currently sessions can be signed by anyone.
    CookieSession::signed(&[0u8; 32])
        .name("s")
        .http_only(true)
        .same_site(SameSite::Lax)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = config::load(None);

    info!("Creating database pool");
    let size = config.database.pool_size.unwrap_or(1);
    let db_pool = create_db_pool(&config.database.url, size);

    info!("Creating server");
    HttpServer::new(move || {
        App::new()
            .data(AppData {
                db_pool: db_pool.clone(),
            })
            .wrap(context::ContextTransform)
            .wrap(make_session_middleware())
            .service(controller::buildings::build)
            .service(controller::buildings::get)
            .service(controller::create_homeworld::get)
            .service(controller::create_homeworld::post)
            .service(controller::home::get)
            .service(controller::join::get)
            .service(controller::join::post)
            .service(controller::overview::get)
            .service(controller::sign_in::post)
    })
    .bind(format!("{}:{}", config.server.address, config.server.port))?
    .run()
    .await
}
