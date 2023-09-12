use crate::cmd::jwt::refresh::Refresh;
use crate::cmd::jwt::verify::Verify;
use crate::cmd::Command;
use crate::state::State;
use avocado_proto::grpc::jwt::jwt_server::Jwt;
use avocado_proto::grpc::jwt::{RefreshReply, RefreshRequest, VerifyReply, VerifyRequest};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub(crate) struct Service {
    pub(crate) state: State,
}

#[tonic::async_trait]
impl Jwt for Service {
    async fn verify(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyReply>, Status> {
        let cmd = Verify {
            token: request.get_ref().token.clone(),
        };
        match cmd.execute(self.state.clone()).await {
            Ok(c) => Ok(Response::new(VerifyReply {
                sub: c.sub,
                exp: c.exp,
                iat: c.iat,
                nbf: c.nbf,
            })),
            Err(e) => Err(e.into()),
        }
    }

    async fn refresh(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshReply>, Status> {
        let cmd = Refresh {
            refresh_token: request.get_ref().refresh_token.clone(),
        };
        match cmd.execute(self.state.clone()).await {
            Ok((access_token, refresh_token)) => Ok(Response::new(RefreshReply {
                access_token,
                refresh_token,
            })),
            Err(e) => Err(e.into()),
        }
    }
}
