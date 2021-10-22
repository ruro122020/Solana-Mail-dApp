// This module will contain the API definition of our program. 
  
impl MailInstruction {
  use crate::error::MailError::InvalidInstruction;
  use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum MailInstruction {
  /// Initialize a new account
  ///
  /// Accounts expected
  ///
  /// 1. `[writable]` The account to be initialized. 
  //      The account is the PDA generated for the user.
  //InitAccount endpoint
  InitAccount,

  /// Send a mail to an account.
  ///
  /// Accounts expected:
  ///
  /// 1. `[writable]` The AccountInfo of the sender
  /// 2. `[writable]` The AccountInfo of the receiver
  // SendMail endpoint
  SendMail { mail: Mail },


}

impl MailInstruction {
        //`input: &[8]` is the instruction data argument passed to our entrypoint function
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
       //tag is assigned the first element returned from split_first
      //rest is the rest of the elements that was returned from split_first
    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;//The From trait in `error.rs` uses the `?` operator to return an error

    Ok(match tag {
      0 => Self::InitAccount,
      // If the value is 1, the function returns a SendMail enum with the mail instance that was deserialized from the rest of the data.
      1 => Self::SendMail {
          mail: Mail::try_from_slice(&rest)?,
      },
      _ => return Err(InvalidInstruction.into()),
    })
  }
} 



 