use crate::context::Context;
use crate::controller::util::assure_body_access;
use crate::view;
use actix_web::web::Query;
use actix_web::{get, HttpResponse, Result};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct BuildingsQuery {
    pub body: Uuid,
}

#[get("/buildings")]
pub async fn get(query: Query<BuildingsQuery>, context: Context) -> Result<HttpResponse> {
    let body_id = query.body;
    assure_body_access(&context, body_id)?;

    Ok(HttpResponse::Ok().body(view::buildings(&context, body_id)))
}
