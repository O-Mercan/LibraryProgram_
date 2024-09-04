use solana_program::{
    program_error::ProgramError,
    msg,
};

use borsh::BorshDeserialize;
use crate::error::LibraryError::InvalidInstruction;
use crate::state::{
    Book, BookNumber, Configuration, User};

#[derive(Debug, PartialEq)]
pub enum LibraryInstruction{
    CreateUser,
    CreateBook,
    AddBook{data: BookNumber},
    RemoveBook,
    BorrowBook,
    ReturnBook,
    Config,
    InitCounter,
    //CheckAuthority{data: Configuration}
}

impl LibraryInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("unpack");
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        msg!("split");

        Ok(match tag{
            0 =>Self::CreateUser ,
            1 =>Self::CreateBook  ,
            2 =>Self::AddBook{data:BookNumber::try_from_slice(&rest)?}  ,
            3 =>Self::RemoveBook  ,
            4 =>Self::BorrowBook ,
            5 =>Self::ReturnBook ,
            6 => Self::Config ,
            7 => Self::InitCounter,
            //8 => Self::CheckAuthority{data:Configuration::try_from_slice(&rest)?},
            _=> return Err(InvalidInstruction.into()),
            
        })
    }
}