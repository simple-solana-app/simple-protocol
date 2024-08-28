use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SimpleProtocolError {
    #[error("Invalid Signer")]
    InvalidSigner,
    #[error("Account Already Exists")]
    AccountAlreadyExists,
}

impl From<SimpleProtocolError> for ProgramError {
    fn from(e: SimpleProtocolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SimpleProtocolError {
    fn type_of() -> &'static str {
        "Simple Protocol Error"
    }
}

impl PrintProgramError for SimpleProtocolError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            SimpleProtocolError::InvalidSigner => {
                msg!("Invalid Signer")
            }
            SimpleProtocolError::AccountAlreadyExists => {
                msg!("Account Already Exists")
            }
        }
    }
}
