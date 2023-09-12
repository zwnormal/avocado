use crate::cmd::{Command, CommandResult};
use crate::domain::user::{Role, User, UserId};
use crate::state::State;
use anyhow::Result;
use avocado_base::secret::SecretString;
use std::borrow::Cow;
use std::collections::HashMap;
use ulid::Ulid;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate)]
pub(crate) struct Add {
    #[validate(email(message = "invalid email address"))]
    pub(crate) email: String,
    #[validate(length(
        min = 2,
        max = 32,
        message = "length of first name must between 2 to 32"
    ))]
    pub(crate) first_name: String,
    #[validate(length(
        min = 2,
        max = 32,
        message = "length of last name must between 2 to 32"
    ))]
    pub(crate) last_name: String,
    #[validate(custom = "validate_password")]
    pub(crate) password: SecretString,
    pub(crate) role: Role,
}

fn validate_password(password: &SecretString) -> Result<(), ValidationError> {
    let password = password.expose_secret();
    if password.len() <= 6 {
        return Err(ValidationError {
            code: Cow::from("password"),
            message: Some(Cow::from("length of password needs to be at least 6")),
            params: HashMap::new(),
        });
    }
    Ok(())
}

#[tonic::async_trait]
impl Command for Add {
    type R = CommandResult<UserId>;

    #[tracing::instrument(name = "Executing 'user add' command", skip(state))]
    async fn execute(&self, state: State) -> Self::R {
        self.validate()?;

        let user = User {
            id: Ulid::new(),
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            password_hash: Self::hash_password(self.password.clone()).await?,
            role: self.role.clone(),
        };
        Ok(state.user_store.insert(user).await?)
    }
}

impl Add {
    async fn hash_password(plain_password: SecretString) -> Result<String> {
        tokio::task::spawn_blocking(move || User::hash_password(plain_password)).await?
    }
}
