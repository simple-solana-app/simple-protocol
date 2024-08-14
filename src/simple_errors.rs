use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimpleProtocolErrors {
    #[error("Max simple already drained")]
    MaxSimpleDrained,
}

impl From<SimpleProtocolErrors> for ProgramError {
    fn from(e: SimpleProtocolErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}
