use crate::context::Context;
use crate::controller::util::assure_body_access;
use crate::view;
use actix_web::web::Query;
use actix_web::{get, HttpResponse, Result};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct OverviewQuery {
    pub body: Uuid,
}

#[get("/overview")]
pub async fn get(query: Query<OverviewQuery>, context: Context) -> Result<HttpResponse> {
    let body_id = query.body;
    assure_body_access(&context, body_id)?;

    Ok(HttpResponse::Ok().body(view::overview(&context, body_id)))
}
