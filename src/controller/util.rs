use crate::context::Context;
use actix_web::error::BlockingError;
use actix_web::{error, web, Error, HttpResponse};
use uuid::Uuid;

pub async fn block<F, I>(f: F) -> Result<I, Error>
where
    F: FnOnce() -> I + Send + 'static,
    I: Send + 'static,
{
    web::block(|| Ok(f()))
        .await
        .map_err(|_err: BlockingError<()>| HttpResponse::InternalServerError().finish().into())
}

pub fn assure_body_access(context: &Context, body_id: Uuid) -> Result<(), Error> {
    if context.bodies().contains_key(&body_id) {
        Ok(())
    } else {
        Err(error::ErrorImATeapot("I'm a teapot"))
    }
}
