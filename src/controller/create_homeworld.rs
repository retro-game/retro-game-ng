use crate::context::Context;
use crate::controller::util::block;
use crate::service::{body, galaxy};
use crate::validation::coordinates;
use crate::view;
use crate::AppData;
use actix_web::http::header::LOCATION;
use actix_web::web::Form;
use actix_web::{get, post, web, Error, HttpResponse};
use log::warn;
use serde::Deserialize;

#[get("/create-homeworld")]
pub async fn get(app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let slots = block(move || galaxy::get_system(app_data, 1, 1)).await?;
    let resp = HttpResponse::Ok().body(view::create_homeworld(&slots));
    Ok(resp)
}

#[derive(Deserialize)]
pub struct CreateHomeworldForm {
    galaxy: i32,
    system: i32,
    position: i32,
}

#[post("/create-homeworld")]
pub async fn post(
    app_data: web::Data<AppData>,
    form: Form<CreateHomeworldForm>,
    context: Context,
) -> Result<HttpResponse, Error> {
    let galaxy = form.galaxy;
    let system = form.system;
    let position = form.position;

    // A special case for the position component, as we cannot create a homeworld on any position.
    // Only 4-12 are allowed.
    if !coordinates::validate_galaxy(galaxy)
        || !coordinates::validate_system(system)
        || position < 4
        || position > 12
    {
        warn!(
            "Creating a homeworld failed, invalid coordinates: user_id={}",
            context.user_id(),
        );

        let resp = HttpResponse::Found()
            .header(LOCATION, "/create-homeworld?error")
            .finish();
        return Ok(resp);
    }

    let user_id = context.user_id();

    let res =
        block(move || body::create_homeworld(app_data, user_id, galaxy, system, position)).await?;
    let location = match res {
        Ok(body_id) => format!("/overview?body={}", body_id),
        Err(_) => "/create-homeworld?error".to_owned(),
    };
    let resp = HttpResponse::Found()
        .header(LOCATION, location.as_str())
        .finish();
    Ok(resp)
}
