use crate::cmd::{Command, CommandResult};
use crate::domain::user::User;
use crate::state::State;

#[derive(Debug)]
pub(crate) struct List;

#[tonic::async_trait]
impl Command for List {
    type R = CommandResult<Vec<User>>;

    #[tracing::instrument(name = "Executing 'user list' command", skip(state))]
    async fn execute(&self, state: State) -> Self::R {
        Ok(state.user_store.list().await?)
    }
}
