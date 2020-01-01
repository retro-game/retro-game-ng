use crate::context::Context;
use crate::controller::util::{assure_body_access, block};
use crate::model::BuildingKind;
use crate::service::buildings;
use crate::view;
use crate::AppData;
use actix_web::http::header::LOCATION;
use actix_web::web::{Data, Form, Query};
use actix_web::{get, post, HttpResponse, Result};
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

#[derive(Deserialize)]
pub struct BuildForm {
    pub body: Uuid,
    pub kind: BuildingKind,
}

#[post("/buildings/build")]
pub async fn build(
    app_data: Data<AppData>,
    form: Form<BuildForm>,
    context: Context,
) -> Result<HttpResponse> {
    let bodies = context.bodies();
    let body = bodies.get(&form.body).unwrap().clone();
    let kind = form.kind;

    block(move || buildings::build(app_data, body, kind)).await?;

    let location = format!("/buildings?body={}", form.body);
    let resp = HttpResponse::Found()
        .header(LOCATION, location.as_str())
        .finish();
    Ok(resp)
}
