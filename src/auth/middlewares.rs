use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};

use super::{service::verify, Claims};

pub struct JwtVerifier;

impl<S, B> Transform<S, ServiceRequest> for JwtVerifier
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtVerifierMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtVerifierMiddleware { service: std::rc::Rc::new(service) })
    }
}

pub struct JwtVerifierMiddleware<S> {
    service: std::rc::Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtVerifierMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let token = req.headers().get("Authorization");

        if let Some(token) = token {
            let token_str = token.to_str().unwrap();
            let claims = verify(token_str).unwrap();
            req.extensions_mut().insert::<Claims>(Claims {
                id: claims.get("id").unwrap().to_string(),
                name: claims.get("name").unwrap().to_string(),
                email: claims.get("email").unwrap().to_string(),
            });
        }

        let service = self.service.clone();

        Box::pin(async move {
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
