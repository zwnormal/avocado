use crate::cmd::{Command, CommandResult};
use crate::session::Session;
use crate::state::State;

#[derive(Debug)]
pub(crate) struct Logout {
    pub(crate) session: Session,
}

#[tonic::async_trait]
impl Command for Logout {
    type R = CommandResult<()>;

    #[tracing::instrument(name = "Executing 'user logout' command", skip(self, state))]
    async fn execute(&self, state: State) -> Self::R {
        state.session_store.logout(&self.session.user_id).await?;
        Ok(())
    }
}
