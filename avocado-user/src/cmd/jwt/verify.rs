use crate::cmd::{Command, CommandResult};
use crate::domain::jwt::Claims;
use crate::state::State;

#[derive(Debug)]
pub(crate) struct Verify {
    pub(crate) token: String,
}

#[tonic::async_trait]
impl Command for Verify {
    type R = CommandResult<Claims>;

    #[tracing::instrument(name = "Executing 'jwt verify' command", skip(self, state))]
    async fn execute(&self, state: State) -> Self::R {
        Ok(Claims::from_jwt_token(
            self.token.clone(),
            state.config.rsa.public_key(),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::jwt::verify::Verify;
    use crate::cmd::Command;
    use crate::db::sqlite::user::Store as UserStore;
    use crate::domain::jwt::Claims;
    use crate::state::State;
    use chrono::Utc;

    #[tokio::test]
    async fn test_verify() {
        let state = State::new(UserStore::new().await);
        let now = Utc::now().timestamp();
        let claims = Claims::new(
            "Test".to_string(),
            state.config.jwt.access_token_expire_time().unwrap(),
            now,
        );
        let token = claims
            .clone()
            .into_jwt_token(state.config.rsa.private_key())
            .unwrap();
        let jwt_verify_cmd = Verify { token };
        let verified_claims = jwt_verify_cmd.execute(state).await.unwrap();
        assert_eq!(verified_claims, claims)
    }
}
