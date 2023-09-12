use crate::cmd::CommandError;
use crate::domain::user::{User, UserError};
use avocado_base::error::ValidationMessages;
use avocado_proto::grpc::user::UserReply;
use tonic::Status;
use validator::ValidationErrors;

impl From<CommandError> for Status {
    fn from(error: CommandError) -> Self {
        tracing::error!("Command Error: {:?}", error.0);

        if let Some(e) = error.0.downcast_ref::<ValidationErrors>() {
            Status::invalid_argument(
                serde_json::to_string(&ValidationMessages::from(e.clone()).messages).unwrap(),
            )
        } else {
            match error.0.downcast_ref::<UserError>() {
                Some(UserError::AuthenticationError) => {
                    Status::unauthenticated(error.0.to_string())
                }
                Some(UserError::NotExist { .. }) => Status::not_found(error.0.to_string()),
                None => Status::internal(error.0.to_string()),
            }
        }
    }
}

impl From<User> for UserReply {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            role: user.role.to_string(),
        }
    }
}

pub(crate) mod service;
