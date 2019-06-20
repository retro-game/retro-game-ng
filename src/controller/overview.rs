use crate::view;
use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OverviewQuery {
    pub body: i64,
}

#[get("/overview")]
pub fn get(query: web::Query<OverviewQuery>) -> HttpResponse {
    HttpResponse::Ok().body(view::overview())
}
