use actix_web::error::BlockingError;
use actix_web::{web, Error, HttpResponse};
use futures::Future;

pub fn block<F, I>(f: F) -> impl Future<Item = I, Error = Error>
where
    F: FnOnce() -> I + Send + 'static,
    I: Send + 'static,
{
    web::block(|| Ok(f()))
        .map_err(|_err: BlockingError<()>| HttpResponse::InternalServerError().finish().into())
}
