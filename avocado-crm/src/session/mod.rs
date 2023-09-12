use avocado_proto::grpc::jwt::jwt_client::JwtClient;
use avocado_proto::grpc::jwt::{VerifyReply, VerifyRequest};
use chrono::{DateTime, Utc};
use tonic::transport::Channel;
use ulid::Ulid;
use uuid::Uuid;

pub(crate) type UserId = Ulid;
pub(crate) type SessionId = Uuid;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Session {
    pub(crate) id: SessionId,
    pub(crate) access_token: String,
    pub(crate) access_token_expire_at: DateTime<Utc>,
    pub(crate) refresh_token: String,
    pub(crate) refresh_token_expire_at: DateTime<Utc>,
    pub(crate) login_at: DateTime<Utc>,
    pub(crate) user_id: UserId,
    pub(crate) email: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) role: String,
}

pub async fn verify_jwt_token(
    mut jwt_client: JwtClient<Channel>,
    token: String,
) -> anyhow::Result<VerifyReply> {
    let request = tonic::Request::new(VerifyRequest { token });
    Ok(jwt_client.verify(request).await?.into_inner())
}

pub(crate) mod cmd;
