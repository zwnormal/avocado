use crate::cmd::{Command, CommandResult};
use crate::session::Session;
use crate::state::State;
use avocado_proto::grpc::user::user_client::UserClient;
use avocado_proto::grpc::user::{ListRequest, UserReply};
use futures_util::StreamExt;
use tonic::metadata::MetadataValue;

#[derive(Debug)]
pub(crate) struct List {
    pub(crate) session: Session,
}

#[tonic::async_trait]
impl Command for List {
    type R = CommandResult<Vec<UserReply>>;

    #[tracing::instrument(name = "Executing 'user list' command", skip(self, state))]
    async fn execute(&self, state: State) -> Self::R {
        let mut user_client =
            UserClient::connect(state.config.service_address.user.clone()).await?;

        let access_token: MetadataValue<_> = self.session.access_token.parse()?;
        let mut request = tonic::Request::new(ListRequest {});
        request.metadata_mut().insert("auth", access_token.clone());
        let mut list_reply = user_client.list(request).await?.into_inner();

        let mut users: Vec<UserReply> = vec![];
        while let Some(user) = list_reply.next().await {
            match user {
                Ok(u) => users.push(u),
                Err(e) => tracing::error!("user list command error: {:?}", e),
            };
        }
        Ok(users)
    }
}
