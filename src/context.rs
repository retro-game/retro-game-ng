use crate::db::body;
use crate::model;
use crate::AppData;
use actix_service::{Service, Transform};
use actix_session::UserSession;
use actix_web::dev::{Payload, ServiceRequest, ServiceResponse};
use actix_web::{error, web, Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{err, ok, Future, Ready};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::task;
use std::task::Poll;
use uuid::{Uuid, UuidBytes};

pub struct Context(pub Rc<RefCell<ContextInner>>);

impl Context {
    pub fn user_id(&self) -> Uuid {
        self.0.borrow().user_id
    }

    pub fn bodies(&self) -> Ref<HashMap<Uuid, model::Body>> {
        Ref::map(self.0.borrow(), |inner| &inner.bodies)
    }
}

pub struct ContextInner {
    pub user_id: Uuid,
    pub bodies: HashMap<Uuid, model::Body>,
}

pub struct ContextTransform;

impl<S, B> Transform<S> for ContextTransform
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type InitError = ();
    type Transform = ContextMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ContextMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct ContextMiddleware<S: 'static> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for ContextMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.borrow_mut().poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut service = self.service.clone();

        Box::pin(async move {
            let app_data: web::Data<AppData> = req.app_data().unwrap();
            let session = req.get_session();

            let user_id_opt = session
                .get::<UuidBytes>("u")
                .ok()
                .flatten()
                .map(|bytes| Uuid::from_bytes(&bytes).ok())
                .flatten();
            let user_id = match user_id_opt {
                None => return service.call(req).await,
                Some(user_id) => user_id,
            };

            let bodies = web::block::<_, _, ()>(move || {
                let conn = &app_data.db_pool.get().unwrap();
                let bodies = body::find_all_by_user_id(conn, &user_id);
                Ok(bodies)
            })
            .await?
            .into_iter()
            .fold(HashMap::new(), |mut acc, body| {
                let old_value = acc.insert(body.id, body);
                debug_assert!(old_value.is_none());
                acc
            });

            let inner = Rc::new(RefCell::new(ContextInner { user_id, bodies }));
            req.extensions_mut().insert(inner);

            service.call(req).await
        })
    }
}

impl FromRequest for Context {
    type Error = Error;
    type Future = Ready<Result<Context, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        match extensions.get() {
            // FIXME: Redirect to login page.
            None => err(error::ErrorUnauthorized("Unauthorized access")),
            Some(context_ext) => ok(Context(Rc::clone(&context_ext))),
        }
    }
}
