use std::env;

use axum::{
    body::{Body, BoxBody},
    http::Request,
    response::Response,
};
use futures::future::BoxFuture;
use hyper::{body, Client, Method};
use serde::{Deserialize, Serialize};
use slog::{error, info, Logger};
use tower::{Layer, Service};

#[derive(Serialize, Deserialize)]
pub struct OpaRequest {
    input: OpaInput,
}

#[derive(Serialize, Deserialize)]
pub struct OpaInput {
    token: String,
    path: Vec<String>,
    method: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpaResponse {
    result: bool,
}

#[derive(Debug, Clone)]
pub struct PolicyEnforcerLayer {
    logger: Logger,
}

impl PolicyEnforcerLayer {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }
}

impl<S> Layer<S> for PolicyEnforcerLayer {
    type Service = PolicyEnforcerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PolicyEnforcerService {
            logger: self.logger.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PolicyEnforcerService<S> {
    logger: Logger,
    inner: S,
}

impl<S> PolicyEnforcerService<S> {
    pub fn new(logger: Logger, inner: S) -> Self {
        Self { logger, inner }
    }
}

enum PolicyError {
    InternalServerError,
    AuthorizationError,
    Succeed,
}

impl<S> Service<Request<Body>> for PolicyEnforcerService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        info!(
            self.logger,
            "method={}; path={};",
            req.method(),
            req.uri().path()
        );

        let author_header = req.headers().get("authorization").unwrap();
        let bearer = author_header
            .to_str()
            .unwrap()
            .to_string()
            .replace(&['\''][..], "");

        info!(self.logger, "header=authorization; value={}", bearer);

        let opa_request = OpaRequest {
            input: OpaInput {
                token: bearer,
                path: req
                    .uri()
                    .path()
                    .split("/")
                    .skip(1)
                    .map(|s| s.to_string())
                    .collect(),
                method: req.method().to_string(),
            },
        };

        let opa_request_json = serde_json::to_string(&opa_request).unwrap();

        info!(self.logger, "opa_request_json={}", opa_request_json);
        let log = self.logger.clone();

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let client = Client::new();
            let client_req = Request::builder()
                .method(Method::POST)
                .uri(env::var("OPA_API_SERVER_URL").unwrap())
                .header("content-type", "application/json")
                .body(Body::from(opa_request_json))
                .unwrap();

            let result = client.request(client_req).await;
            let mut error_type: PolicyError = PolicyError::AuthorizationError;

            match result {
                Ok(response) => {
                    let bytes_result = body::to_bytes(response.into_body()).await;
                    match bytes_result {
                        Ok(bytes) => {
                            let body = String::from_utf8(bytes.to_vec()).unwrap();
                            info!(log, "body={}", body);
                            if body == "{}" {
                                error_type = PolicyError::InternalServerError;
                            } else {
                                let opa_response: OpaResponse =
                                    serde_json::from_str(&body).unwrap();
                                info!(log, "opa_response={}", opa_response.result);
                                if opa_response.result {
                                    error_type = PolicyError::Succeed;
                                } else {
                                    error_type = PolicyError::AuthorizationError;
                                }
                            }
                        }
                        Err(e) => {
                            error!(log, "error={}", e);
                            error_type = PolicyError::InternalServerError;
                        }
                    }
                }
                Err(e) => {
                    error!(log, "error={}", e);
                    error_type = PolicyError::InternalServerError;
                }
            };

            match error_type {
                PolicyError::Succeed => {
                    let response = inner.call(req).await?;
                    Ok(response)
                }
                PolicyError::AuthorizationError => {
                    let response = Response::builder()
                        .status(403)
                        .body(BoxBody::default())
                        .unwrap();
                    Ok(response)
                }
                PolicyError::InternalServerError => {
                    let response = Response::builder()
                        .status(500)
                        .body(BoxBody::default())
                        .unwrap();
                    Ok(response)
                }
            }
        })
    }
}
