use crate::db::sqlite::user::Store as UserStore;
use crate::grpc::service::jwt::Service as JwtService;
use crate::grpc::service::user::Service as UserService;
use crate::middleware::auth::AuthLayer;
use crate::state::State;
use avocado_proto::grpc::jwt::jwt_server::JwtServer;
use avocado_proto::grpc::user::user_server::UserServer;
use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;
use tonic::transport::Server;

mod cfg;
mod cmd;
mod db;
mod domain;
pub mod grpc;
mod middleware;
mod state;

pub async fn run(
    address: SocketAddr,
) -> Result<impl Future<Output = Result<(), tonic::transport::Error>>, Box<dyn std::error::Error>> {
    let state = State::new(UserStore::new().await);
    let user_service = UserService {
        state: state.clone(),
    };
    let jwt_service = JwtService {
        state: state.clone(),
    };

    let layer = tower::ServiceBuilder::new()
        .timeout(Duration::from_secs(300))
        .layer(AuthLayer { state })
        .into_inner();

    let server = Server::builder()
        .layer(layer)
        .add_service(UserServer::new(user_service))
        .add_service(JwtServer::new(jwt_service))
        .serve(address);
    Ok(server)
}
