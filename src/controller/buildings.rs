use crate::context::Context;
use crate::controller::util::assure_body_access;
use crate::service::buildings;
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

    let bodies = context.bodies();
    let body = bodies.get(&body_id).unwrap();
    let buildings_and_queue = buildings::get_buildings_and_queue(&body);

    Ok(HttpResponse::Ok().body(view::buildings(body_id, &buildings_and_queue)))
}
