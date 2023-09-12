use crate::cmd::{Command, CommandResult};
use crate::domain::jwt::Claims;
use crate::domain::user::{User, UserError};
use crate::state::State;
use anyhow::Result;
use avocado_base::secret::SecretString;
use chrono::Utc;
use validator::Validate;

#[derive(Debug, Validate)]
pub(crate) struct Login {
    #[validate(email(message = "invalid email address"))]
    pub(crate) email: String,
    pub(crate) password: SecretString,
}

#[tonic::async_trait]
impl Command for Login {
    type R = CommandResult<(String, String)>;

    #[tracing::instrument(name = "Executing 'user login' command", skip(state))]
    async fn execute(&self, state: State) -> Self::R {
        self.validate()?;

        match state.user_store.get_by_email(self.email.as_str()).await? {
            Some(u)
                if Self::verify_password(self.password.clone(), u.password_hash.clone())
                    .await? =>
            {
                let now = Utc::now().timestamp();
                let claims = Claims::new(
                    u.id.to_string(),
                    state.config.jwt.access_token_expire_time()?,
                    now,
                );
                let access_token = claims.into_jwt_token(state.config.rsa.private_key())?;

                let claims = Claims::new(
                    u.id.to_string(),
                    state.config.jwt.refresh_token_expire_time()?,
                    now,
                );
                let refresh_token = claims.into_jwt_token(state.config.rsa.private_key())?;

                Ok((access_token, refresh_token))
            }
            _ => Err(UserError::AuthenticationError.into()),
        }
    }
}

impl Login {
    async fn verify_password(plain_password: SecretString, password_hash: String) -> Result<bool> {
        tokio::task::spawn_blocking(move || User::verify_password(plain_password, password_hash))
            .await?
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::user::login::Login;
    use crate::cmd::Command;
    use crate::db::sqlite::user::Store as UserStore;
    use crate::state::State;
    use avocado_base::secret::SecretString;

    #[tokio::test]
    async fn test_login() {
        let state = State::new(UserStore::new().await);

        let login_cmd = Login {
            email: "admin@avocado.com".to_string(),
            password: SecretString::new("kIxv4NomLT0WwGKF".to_string()),
        };
        let result = login_cmd.execute(state.clone()).await;
        assert!(result.is_ok());

        let login_cmd = Login {
            email: "admin@avocado.com".to_string(),
            password: SecretString::new("secureitis".to_string()),
        };
        let result = login_cmd.execute(state.clone()).await;
        assert!(result.is_err());

        let login_cmd = Login {
            email: "".to_string(),
            password: SecretString::new("pass".to_string()),
        };
        let result = login_cmd.execute(state.clone()).await;
        assert!(result.is_err())
    }
}
