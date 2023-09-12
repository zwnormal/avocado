use crate::cmd::jwt::verify::Verify;
use crate::cmd::user::get::Get;
use crate::cmd::{Command, CommandResult};
use crate::domain::user::User as DomainUser;
use crate::state::State;

#[derive(Debug)]
pub(crate) struct Who {
    pub(crate) token: String,
}

#[tonic::async_trait]
impl Command for Who {
    type R = CommandResult<DomainUser>;

    #[tracing::instrument(name = "Executing 'jwt who' command", skip(self, state))]
    async fn execute(&self, state: State) -> Self::R {
        let claims = Verify {
            token: self.token.to_string(),
        }
        .execute(state.clone())
        .await?;

        let user_id = claims.get_user_id()?;

        Ok(Get { user_id }.execute(state.clone()).await?)
    }
}
