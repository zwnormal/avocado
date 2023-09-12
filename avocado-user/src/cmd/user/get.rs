use crate::cmd::{Command, CommandResult};
use crate::domain::user::{User, UserError, UserId};
use crate::state::State;

#[derive(Debug)]
pub(crate) struct Get {
    pub(crate) user_id: UserId,
}

#[tonic::async_trait]
impl Command for Get {
    type R = CommandResult<User>;

    #[tracing::instrument(name = "Executing 'user get' command", skip(state))]
    async fn execute(&self, state: State) -> Self::R {
        match state.user_store.get(&self.user_id).await {
            Ok(Some(u)) => Ok(u),
            _ => Err(UserError::NotExist {
                field: "id".to_string(),
                value: self.user_id.to_string(),
            }
            .into()),
        }
    }
}
