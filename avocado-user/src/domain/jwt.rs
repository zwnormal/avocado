use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: i64,
    pub(crate) iat: i64,
    pub(crate) nbf: i64,
}

impl Claims {
    pub(crate) fn new(subject: String, expire_time: i64, issue_at: i64) -> Self {
        Self {
            sub: subject,
            exp: expire_time,
            iat: issue_at,
            nbf: issue_at,
        }
    }

    pub(crate) fn from_jwt_token(token: String, public_key: Vec<u8>) -> Result<Self> {
        let validation = Validation::new(Algorithm::RS256);
        let token = decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_rsa_pem(public_key.as_slice())?,
            &validation,
        )?;
        Ok(token.claims)
    }

    pub(crate) fn into_jwt_token(self, private_key: Vec<u8>) -> Result<String> {
        let header = Header::new(Algorithm::RS256);
        Ok(encode::<Claims>(
            &header,
            &self,
            &EncodingKey::from_rsa_pem(private_key.as_slice())?,
        )?)
    }

    pub(crate) fn get_user_id(&self) -> Result<Ulid> {
        Ok(Ulid::from_string(self.sub.as_str())?)
    }
}
