use std::str::FromStr;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    system_instruction,
    borsh::try_from_slice_unchecked,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    program_pack::IsInitialized,
};
use borsh::BorshSerialize;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program::invoke;
use crate::ADMIN_PUBKEY;
use crate::projects::state::{PROJECT_ACCOUNT_DATA_VERSION, ProjectAccountState};
use crate::error::CandyMachineAffiliatesError;
use crate::utils::validate_client_pda;

#[allow(unused_variables)]
pub fn close_project_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
) -> ProgramResult {
    msg!("Closing project account...");

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let owner_account = next_account_info(account_info_iter)?;

    // Validate accounts

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let admin_pubkey = Pubkey::from_str(ADMIN_PUBKEY).expect("Pubkey conversion failed");

    if initializer.key.ne(&admin_pubkey) {
        msg!("Action not allowed");
        return Err(CandyMachineAffiliatesError::ActionNotAllowed.into());
    }

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner)
    }

    let (client_pda_is_valid, bump_seed) = validate_client_pda(
        pda_account,
        &[
            ProjectAccountState::DISCRIMINATOR.as_ref(),
            project_owner_pubkey.as_ref(),
            candy_machine_id.as_ref(),
        ],
        program_id
    );

    if !client_pda_is_valid {
        msg!("Invalid seeds for PDA");
        return Err(CandyMachineAffiliatesError::InvalidPDA.into());
    }

    msg!("Unpacking project account state");
    let project_account_state = try_from_slice_unchecked::<ProjectAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Borrowed project account data");

    msg!("Checking if project account is initialized");
    if !project_account_state.is_initialized() {
        msg!("Project account not initialized");
        return Err(CandyMachineAffiliatesError::UninitializedAccount.into());
    }

    **owner_account.try_borrow_mut_lamports()? += pda_account.lamports();
    **pda_account.try_borrow_mut_lamports()? = 0;
    *pda_account.try_borrow_mut_data()? = &mut [];

    msg!("Project account closed.");

    Ok(())
}
