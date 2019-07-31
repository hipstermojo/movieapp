use argonautica::{Error, Hasher,Verifier};
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ExistingUserError;

impl fmt::Display for ExistingUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User already exists")
    }
}

impl error::Error for ExistingUserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "User already exists"
    }
}

#[derive(Debug)]
pub enum HandlerErrors {
    UserNotExistError,
    HashingError,
    ValidationError(ExistingUserError),
    DatabaseError(mongodb::Error),
    DecoderError(mongodb::DecoderError)
}

pub fn encrypt_password(password: &str) -> Result<String, Error> {
    let mut hasher = Hasher::default();
    hasher
        .with_password(password)
        .with_secret_key("Super secret")
        .hash()
}

pub fn verify_password(hash: &str,password: &str) -> Result<bool,Error> {
    let mut verifier = Verifier::default();
    verifier.with_hash(hash).with_password(password).with_secret_key("Super secret").verify()
}
