use crate::cmd::user::add::Add;
use crate::cmd::user::list::List;
use crate::cmd::user::login::Login;
use crate::cmd::Command;
use crate::domain::user::Role;
use crate::domain::user::User as DomainUser;
use crate::state::State;
use avocado_base::secret::SecretString;
use avocado_proto::grpc::user::user_server::User;
use avocado_proto::grpc::user::{
    AddReply, AddRequest, ListRequest, LoginReply, LoginRequest, UserReply, WhoAmIRequest,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub(crate) struct Service {
    pub(crate) state: State,
}

#[tonic::async_trait]
impl User for Service {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        let cmd = Login {
            email: request.get_ref().email.clone(),
            password: SecretString::new(request.get_ref().password.clone()),
        };
        match cmd.execute(self.state.clone()).await {
            Ok((access_token, refresh_token)) => Ok(Response::new(LoginReply {
                access_token,
                refresh_token,
            })),
            Err(e) => Err(e.into()),
        }
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddReply>, Status> {
        return if let Ok(role) = TryInto::<Role>::try_into(request.get_ref().role) {
            let cmd = Add {
                email: request.get_ref().email.clone(),
                first_name: request.get_ref().first_name.clone(),
                last_name: request.get_ref().last_name.clone(),
                password: SecretString::new(request.get_ref().password.clone()),
                role,
            };
            match cmd.execute(self.state.clone()).await {
                Ok(user_id) => Ok(Response::new(AddReply {
                    user_id: user_id.to_string(),
                })),
                Err(e) => Err(e.into()),
            }
        } else {
            Err(Status::invalid_argument("invalid user rol"))
        };
    }

    type ListStream = ReceiverStream<Result<UserReply, Status>>;

    async fn list(
        &self,
        _request: Request<ListRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let users = List.execute(self.state.clone()).await?;
        let (tx, rx) = mpsc::channel(8);

        tokio::spawn(async move {
            for user in users {
                match tx.send(Ok(user.into())).await {
                    Ok(_) => continue,
                    Err(e) => {
                        tracing::error!("user list channel sending error: {:?}", e)
                    }
                };
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn who_am_i(
        &self,
        request: Request<WhoAmIRequest>,
    ) -> Result<Response<UserReply>, Status> {
        if let Some(user) = request.extensions().get::<DomainUser>() {
            Ok(Response::new(user.clone().into()))
        } else {
            Err(Status::unauthenticated("user not found"))
        }
    }
}
