use std::rc::Rc;

use futures_core::future::LocalBoxFuture;

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use ory_kratos_client::{
    apis::{
        configuration::Configuration as KratosConfiguration,
        v0alpha1_api::{self},
    },
    models::Session,
};

trait KratosSession {}
impl KratosSession for Session {}

#[derive(Debug, Clone)]
pub struct KratosIdentity {
    pub configuration: KratosConfiguration,
}

impl KratosIdentity {
    pub fn new(configuration: KratosConfiguration) -> Self {
        Self { configuration }
    }
}

impl<S, B> Transform<S, ServiceRequest> for KratosIdentity
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = KratosIdentityMiddleware<S>;
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let configuration = self.configuration.clone();
        Box::pin(async move {
            Ok(KratosIdentityMiddleware {
                service: Rc::new(service),
                configuration,
            })
        })
    }
}

pub struct KratosIdentityMiddleware<S> {
    // Wrap service in Rc to make it cloneable
    pub(crate) service: Rc<S>,
    pub(crate) configuration: KratosConfiguration,
}

impl<S, B> Service<ServiceRequest> for KratosIdentityMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let configuration = self.configuration.clone();
        let cookie_header = req.headers().get("cookie").unwrap();
        let cookie = cookie_header.to_str().expect("Invalid cookie.").to_string();

        println!("Something");

        let svc = Rc::clone(&self.service);
        Box::pin(async move {
            let sess = v0alpha1_api::to_session(&configuration, None, Some(cookie.as_str())).await;
            if sess.is_ok() {
                if sess.as_ref().unwrap().active.unwrap() {
                    svc.call(req).await
                } else {
                    svc.call(req).await
                }
            } else {
                svc.call(req).await
            }
        })
    }
}
