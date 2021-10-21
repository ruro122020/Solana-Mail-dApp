use crate::error::MailError::NotWritable;
use crate::instruction::MailInstruction;
use crate::state::{Mail, MailAccount};
use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey;
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = MailInstruction::unpack(instruction_data)?;

        match instruction {
            MailInstruction::InitAccount => {
                msg!("Instruction: InitAccount");
                Self::process_init_account(accounts, program_id)
            }
        }
    }
    //AccountInfo is the account info of the PDA address of the user
    //Pubkey is the program id. 
    fn process_init_account(
        account: &AccountInfo,
        program_id: &Pubkey
    ) -> ProgramResult {
        // check whether the account is writable
        // return an error if it's not
        if !account.is_writable{
            //NotWriteable is a custom error in MailError struct in error.rs
            return Err(NotWritable.into());
        }
        // confirm the account owner is the same as the program id
        // return an error if it's not
        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let welcom_mail = Mail {
            id: String::from("00000000-0000-0000-0000-000000000000"),
            from_address: program_id.to_string(),
            to_address: account.key.to_string(),
            subject: String::from("Welcome to SolMail"),
            body: String::from("This is the start of your private messages on SolMail
            Lorem, ipsum dolor sit amet consectetur adipisicing elit. Quos ut labore, debitis assumenda, dolorem nulla facere soluta exercitationem excepturi provident ipsam reprehenderit repellat quisquam corrupti commodi fugiat iusto quae voluptates!"),
            sent_date: "9/29/2021, 3:58:02 PM"
        };

        let mail_account = MailAccount {
            inbox: vec![welcome_mail],
            sent: Vec::new();
        };

        // the serialize() method takes a reference to a mutable slice of u8 as an arugment
        // the borrow_mut() method returns a `RefMut`
        // Since we can't pass RerMut to a method that expects a slice, we take a mutable slice of `RefMut` which returns a mutable slice of u8
        mail_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
        Ok(())
    }
}