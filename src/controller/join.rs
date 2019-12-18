use crate::controller::util::block;
use crate::service::join;
use crate::service::join::ErrorFlags;
use crate::view;
use crate::AppData;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, Error, HttpResponse};
use serde::Deserialize;

#[get("/join")]
pub fn get() -> HttpResponse {
    let body = view::join(ErrorFlags::empty());
    HttpResponse::Ok().body(body)
}

#[derive(Deserialize)]
pub struct JoinForm {
    email: String,
    name: String,
    password: String,
    #[serde(rename = "password-confirm")]
    password_confirm: String,
}

#[post("/join")]
pub async fn post(
    app_data: web::Data<AppData>,
    form: web::Form<JoinForm>,
) -> Result<HttpResponse, Error> {
    let error_flags = block(move || {
        join::join(
            app_data,
            &form.email,
            &form.name,
            &form.password,
            &form.password_confirm,
        )
    })
    .await?;
    let resp = if !error_flags.is_empty() {
        let body = view::join(error_flags);
        HttpResponse::Ok().body(body)
    } else {
        let location = "/?joined";
        HttpResponse::Found().header(LOCATION, location).finish()
    };
    Ok(resp)
}
