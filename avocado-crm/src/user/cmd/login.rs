use crate::cmd::Command;
use crate::session::Session;
use crate::session::{verify_jwt_token, SessionId};
use crate::state::State;
use anyhow::{anyhow, Result};
use avocado_base::secret::SecretString;
use avocado_proto::grpc::jwt::jwt_client::JwtClient;
use avocado_proto::grpc::user::user_client::UserClient;
use avocado_proto::grpc::user::{LoginRequest, UserReply, WhoAmIRequest};
use chrono::{TimeZone, Utc};
use serde::Deserialize;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use ulid::Ulid;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub(crate) struct Login {
    pub(crate) email: String,
    pub(crate) password: SecretString,
}

#[tonic::async_trait]
impl Command for Login {
    type R = Result<SessionId>;

    #[tracing::instrument(name = "Executing 'user login' command", skip(state))]
    async fn execute(&self, state: State) -> Self::R {
        let mut user_client =
            UserClient::connect(state.config.service_address.user.clone()).await?;
        let request = tonic::Request::new(LoginRequest {
            email: self.email.clone(),
            password: self.password.expose_secret().clone(),
        });
        let login_reply = user_client.login(request).await?.into_inner();

        let jwt_client = JwtClient::connect(state.config.service_address.user.clone()).await?;
        let access_token_verify_reply =
            verify_jwt_token(jwt_client.clone(), login_reply.access_token.clone()).await?;
        let refresh_token_verify_reply =
            verify_jwt_token(jwt_client, login_reply.refresh_token.clone()).await?;

        let who_reply = who_i_am(user_client, login_reply.access_token.clone()).await?;
        let session_id = Uuid::new_v4();
        state
            .session_store
            .insert(Session {
                id: session_id,
                access_token: login_reply.access_token,
                access_token_expire_at: Utc
                    .timestamp_opt(access_token_verify_reply.exp, 0)
                    .single()
                    .ok_or(anyhow!("cannot get access token expire time"))?,
                refresh_token: login_reply.refresh_token,
                refresh_token_expire_at: Utc
                    .timestamp_opt(refresh_token_verify_reply.exp, 0)
                    .single()
                    .ok_or(anyhow!("cannot get refresh token expire time"))?,
                login_at: Utc::now(),
                user_id: Ulid::from_string(who_reply.id.as_str())?,
                email: who_reply.email,
                first_name: who_reply.first_name,
                last_name: who_reply.last_name,
                role: who_reply.role,
            })
            .await?;
        Ok(session_id)
    }
}

async fn who_i_am(mut user_client: UserClient<Channel>, access_token: String) -> Result<UserReply> {
    let access_token: MetadataValue<_> = access_token.parse()?;
    let mut request = tonic::Request::new(WhoAmIRequest {});
    request.metadata_mut().insert("auth", access_token.clone());
    Ok(user_client.who_am_i(request).await?.into_inner())
}
