use crate::view;
use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HomeQuery {
    error: Option<String>,
    joined: Option<String>,
}

#[get("/")]
pub fn get(query: web::Query<HomeQuery>) -> HttpResponse {
    let sign_in_error = query.error.is_some();
    let joined = query.joined.is_some();
    HttpResponse::Ok().body(view::home(sign_in_error, joined))
}
