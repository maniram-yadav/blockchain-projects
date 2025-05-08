use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MathOperation {
    pub square : u32,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data :&[u8]
) -> ProgramResult {
    let accounts_iter =  &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    msg!("Debug Output:");
    msg!("Account Id : {} ",account.key);
    msg!("Executable ? : {} ",account.executable);
    msg!("Lamports : {:#?} ",account.lamports);
    msg!("Debug output complete.");
    msg!("Squaring the provided value");

    let mut math_operation = MathOperation::try_from_slice(&account.data.borrow())?;
    math_operation.square = math_operation.square.pow(2);
    math_operation.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Current Square is now: {}", math_operation.square);
    Ok(())
}
