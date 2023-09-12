use chrono::Utc;

use crate::cmd::jwt::who::Who;
use crate::cmd::{Command, CommandResult};
use crate::domain::jwt::Claims;
use crate::state::State;

#[derive(Debug)]
pub(crate) struct Refresh {
    pub(crate) refresh_token: String,
}

#[tonic::async_trait]
impl Command for Refresh {
    type R = CommandResult<(String, String)>;

    #[tracing::instrument(name = "Executing 'jwt refresh' command", skip(self, state))]
    async fn execute(&self, state: State) -> Self::R {
        let user = Who {
            token: self.refresh_token.clone(),
        }
        .execute(state.clone())
        .await?;
        let now = Utc::now().timestamp();
        let access_token = Claims::new(
            user.id.to_string(),
            state.config.jwt.access_token_expire_time()?,
            now,
        )
        .into_jwt_token(state.config.rsa.private_key())?;
        let refresh_token = Claims::new(
            user.id.to_string(),
            state.config.jwt.refresh_token_expire_time()?,
            now,
        )
        .into_jwt_token(state.config.rsa.private_key())?;
        Ok((access_token, refresh_token))
    }
}
