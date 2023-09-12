use crate::cmd::jwt::who::Who;
use crate::cmd::Command;
use crate::state::State;
use futures_util::future::BoxFuture;
use hyper::Response;
use hyper::StatusCode;
use std::task::{Context, Poll};
use tonic::body::{empty_body, BoxBody};
use tonic::transport::Body;
use tonic::Code;
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub(crate) struct AuthLayer {
    pub(crate) state: State,
}

#[derive(Debug, Clone)]
pub(crate) struct Auth<S> {
    state: State,
    inner: S,
}

impl<S> Layer<S> for AuthLayer {
    type Service = Auth<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Auth {
            state: self.state.clone(),
            inner,
        }
    }
}

impl<S> Service<hyper::Request<Body>> for Auth<S>
where
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<Body>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let state = self.state.clone();

        Box::pin(async move {
            let path = req.uri().path();
            if path != "/user.User/Login" && path != "/jwt.Jwt/Verify" {
                match req.headers().get("auth").and_then(|t| t.to_str().ok()) {
                    Some(token) => {
                        let user = Who {
                            token: token.to_string(),
                        }
                        .execute(state)
                        .await;
                        match user {
                            Ok(u) => {
                                req.extensions_mut().insert(u);
                                let response = inner.call(req).await?;
                                Ok(response)
                            }
                            Err(_) => Ok(unauthenticated_response()),
                        }
                    }
                    None => Ok(unauthenticated_response()),
                }
            } else {
                let response = inner.call(req).await?;
                Ok(response)
            }
        })
    }
}

fn unauthenticated_response() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("grpc-status", (Code::Unauthenticated as u8).to_string())
        .body(empty_body())
        .unwrap()
}
