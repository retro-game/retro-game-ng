use crate::controller::util::block;
use crate::service::sign_in::sign_in;
use crate::AppData;
use actix_web::http::header::LOCATION;
use actix_web::{post, web, Error, HttpResponse};
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInForm {
    email: String,
    password: String,
}

#[post("/sign-in")]
pub fn post(
    app_data: web::Data<AppData>,
    form: web::Form<SignInForm>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    block(move || sign_in(app_data, &form.email, &form.password)).and_then(|signed_in| {
        let location = if signed_in {
            "/overview?body=1"
        } else {
            "/?error"
        };
        let resp = HttpResponse::Found().header(LOCATION, location).finish();
        Ok(resp)
    })
}
