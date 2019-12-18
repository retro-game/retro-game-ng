use crate::controller::util::block;
use crate::service::sign_in::sign_in;
use crate::AppData;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{post, web, Error, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInForm {
    email: String,
    password: String,
}

#[post("/sign-in")]
pub async fn post(
    app_data: web::Data<AppData>,
    form: web::Form<SignInForm>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let data_opt = block(move || sign_in(app_data, &form.email, &form.password)).await?;
    let location = if let Some(data) = data_opt {
        // Signing in was successful.
        session.set("u", data.user_id.as_bytes()).unwrap();

        if let Some(homeworld_id) = data.homeworld_id {
            "/overview?body=".to_owned() + &homeworld_id.to_string()
        } else {
            "/create-homeworld".to_owned()
        }
    } else {
        "/?error".to_owned()
    };

    let resp = HttpResponse::Found()
        .header(LOCATION, location.as_str())
        .finish();
    Ok(resp)
}
