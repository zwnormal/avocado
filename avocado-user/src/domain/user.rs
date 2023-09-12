use anyhow::Result;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use avocado_base::secret::SecretString;
use std::fmt;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use ulid::Ulid;

pub(crate) type UserId = Ulid;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Role {
    NormalUser = 0,
    Admin = 1,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Role::NormalUser => write!(f, "user"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

impl TryFrom<i32> for Role {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Role::NormalUser),
            1 => Ok(Role::Admin),
            _ => Err("invalid user role"),
        }
    }
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("fail to authenticate the user")]
    AuthenticationError,
    #[error("user with {field} {value} not exist")]
    NotExist { field: String, value: String },
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct User {
    pub(crate) id: UserId,
    pub(crate) email: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) password_hash: String,
    pub(crate) role: Role,
}

impl User {
    pub(crate) fn verify_password(
        plain_password: SecretString,
        password_hash: String,
    ) -> Result<bool> {
        Ok(Argon2::default()
            .verify_password(
                plain_password.expose_secret().as_bytes(),
                &PasswordHash::new(&password_hash)?,
            )
            .is_ok())
    }

    pub(crate) fn hash_password(plain_password: SecretString) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Argon2::default()
            .hash_password(plain_password.expose_secret().as_bytes(), &salt)?
            .to_string())
    }
}
