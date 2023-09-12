use crate::state::State;

#[derive(Debug)]
pub(crate) struct CommandError(pub(crate) anyhow::Error);

impl<E> From<E> for CommandError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub(crate) type CommandResult<T> = Result<T, CommandError>;

#[tonic::async_trait]
pub(crate) trait Command {
    type R;
    async fn execute(&self, state: State) -> Self::R;
}

pub(crate) mod jwt;
pub(crate) mod user;
