use crate::state::State;

pub(crate) type CommandResult<T> = Result<T, anyhow::Error>;

#[tonic::async_trait]
pub(crate) trait Command {
    type R;
    async fn execute(&self, state: State) -> Self::R;
}
