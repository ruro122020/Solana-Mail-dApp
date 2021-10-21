//import processor module
//Processor is a struct from the processor module
//the Processor struct has a process function
use crate::processor::Processor;
//import solana_program crate
use solana_program::{
  account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);
//this function will be the first function to run any time a request to the program is made
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //the process function will be handling all the requests that comes to our program
    Processor::process(program_id, accounts,instruction_data)
}

