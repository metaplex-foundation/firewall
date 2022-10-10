use saf_macros::*;
use thiserror::Error;

#[derive(Error, Debug, ProgramErrorCode)]
pub enum AccountsError {
    #[error("There is no account here")]
    OutOfAccounts,
    #[error("Expected a required account here")]
    RequiredAccountMissing,
    #[error("Keys are expected to be equal")]
    KeyMismatch,
    #[error("Validation Error: {0}")]
    ValidationError(String),
}
