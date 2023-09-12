use crate::db::sqlite::session::Store as SessionStore;
use crate::middleware::auth::auth;
use crate::state::State;
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum::response::IntoResponse;
use axum::routing::{get, post, IntoMakeService};
use axum::Router;
use hyper::server::conn::AddrIncoming;
use hyper::Server;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod cfg;
mod cmd;
mod db;
mod err;
mod middleware;
mod session;
mod state;
mod user;

pub async fn run(address: SocketAddr) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let state = State::new(SessionStore::new().await);
    let layer = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(from_fn_with_state(state.clone(), auth));

    let app = Router::new()
        .route("/health-check", get(health_check))
        .route("/api/user/login", post(user::json::login::login))
        .route("/api/user/logout", post(user::json::logout::logout))
        .route("/api/user/list", get(user::json::list::list))
        .layer(layer)
        .with_state(state);

    axum::Server::bind(&address).serve(app.into_make_service())
}

async fn health_check() -> axum::response::Response {
    StatusCode::OK.into_response()
}
