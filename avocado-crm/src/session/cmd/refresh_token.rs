use crate::session::{verify_jwt_token, Session};
use crate::state::State;
use anyhow::anyhow;
use anyhow::Result;
use avocado_proto::grpc::jwt::jwt_client::JwtClient;
use avocado_proto::grpc::jwt::RefreshRequest;
use chrono::{DateTime, TimeZone, Utc};
use tonic::metadata::MetadataValue;

pub(crate) struct RefreshToken {
    pub(crate) session: Session,
}

impl RefreshToken {
    pub(crate) async fn execute(mut self, state: State) -> Result<()> {
        if tokens_expired(&self.session) {
            state.session_store.delete(&self.session.id).await?;
            return Err(anyhow!(
                "refresh token expires, cannot update access any longer"
            ));
        }

        if (self.session.access_token_expire_at - Utc::now()).num_seconds() < 120 {
            // If access token nearly expires, ask the user service to refresh
            let (access_token, access_token_exp, refresh_token, refresh_token_exp) =
                refresh_tokens(
                    state.config.service_address.user.clone(),
                    self.session.access_token.clone(),
                    self.session.refresh_token.clone(),
                )
                .await?;

            // Save the updated tokens into the database
            self.session.access_token = access_token;
            self.session.access_token_expire_at = access_token_exp;
            self.session.refresh_token = refresh_token;
            self.session.refresh_token_expire_at = refresh_token_exp;
            state.session_store.update(self.session).await?;
        }
        Ok(())
    }
}

fn tokens_expired(session: &Session) -> bool {
    let now = Utc::now();
    (session.access_token_expire_at - now).num_seconds() < 30
        && (session.refresh_token_expire_at - now).num_seconds() < 30
}

async fn refresh_tokens(
    address: String,
    access_token: String,
    refresh_token: String,
) -> Result<(String, DateTime<Utc>, String, DateTime<Utc>)> {
    let mut jwt_client = JwtClient::connect(address).await?;
    let access_token: MetadataValue<_> = access_token.parse()?;
    let mut request = tonic::Request::new(RefreshRequest {
        refresh_token: refresh_token.clone(),
    });
    request.metadata_mut().insert("auth", access_token.clone());
    let refresh_reply = jwt_client.refresh(request).await?.into_inner();
    let access_token_verify_reply =
        verify_jwt_token(jwt_client.clone(), refresh_reply.access_token.clone()).await?;

    let refresh_token_verify_reply =
        verify_jwt_token(jwt_client.clone(), refresh_reply.refresh_token.clone()).await?;
    Ok((
        refresh_reply.access_token,
        Utc.timestamp_opt(access_token_verify_reply.exp, 0)
            .single()
            .ok_or(anyhow!("cannot get access token expire time"))?,
        refresh_reply.refresh_token,
        Utc.timestamp_opt(refresh_token_verify_reply.exp, 0)
            .single()
            .ok_or(anyhow!("cannot get refresh token expire time"))?,
    ))
}
