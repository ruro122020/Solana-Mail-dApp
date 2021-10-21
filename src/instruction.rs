// This module will contain the API definition of our program. 


use crate::error::MailError::InvalidInstruction;
use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum MailInstruction{
    //Initialize a new account
    //`input: &[8]` is the instruction data argument passed to our entrypoint function
    pub fn unpack(input: &[8]) -> Result<Self, ProgramError> {
        //tag is assigned the first element returned from split_first
        //rest is the rest of the elements that was returned from split_first
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?; //The From trait in `error.rs` uses the `?` operator to return an error 

        Ok(match tag {
            0 => Self::InitAccount,
            _ => return Err(InvalidInstruction.into()),
        })
    }
    //Accounts expected

    //1. The AccountInfo of the account to be initialized. This account is writeable.
    // the account is the PDA generated for the user
    InitAccount, 
}