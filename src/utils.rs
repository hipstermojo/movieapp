use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct ExistingUserError;

impl fmt::Display for ExistingUserError {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"User already exists")
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
    ValidationError(ExistingUserError),
    DatabaseError(mongodb::Error),
}