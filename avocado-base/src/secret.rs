use secrecy::ExposeSecret;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct SecretString(secrecy::SecretString);

impl SecretString {
    pub fn new(value: String) -> Self {
        Self(secrecy::SecretString::new(value))
    }

    pub fn expose_secret(&self) -> &String {
        self.0.expose_secret()
    }
}

impl Serialize for SecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("<Secret>")
    }
}

impl<'de> Deserialize<'de> for SecretString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: secrecy::SecretString = Deserialize::deserialize(deserializer)?;
        Ok(Self(s))
    }
}
