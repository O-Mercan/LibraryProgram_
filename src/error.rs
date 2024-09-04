use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum LibraryError{
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Arithmetic Error")]
    ArithmeticError,

    #[error("Payer account is not signer")]
    NotSignerPayer,

    #[error("owner is not program id")]
    InvalidOwner,

    #[error("User is not authority")]
    AuthorityError
}


impl From<LibraryError> for ProgramError {
    fn from(e: LibraryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}