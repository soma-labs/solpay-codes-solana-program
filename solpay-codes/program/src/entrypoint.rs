use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
    msg,
};

use crate::processor;

entrypoint!(process_instruction);

#[allow(unused_variables)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!(
        "Processing instruction: {}, {} accounts, {:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    processor::process_instruction(program_id, accounts, instruction_data)?;

    Ok(())
}
