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
use solana_program::clock::Clock;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program::invoke;
use crate::ADMIN_PUBKEY;
use crate::projects::state::{MAX_PROJECT_TITLE_LENGTH, PROJECT_ACCOUNT_DATA_VERSION, ProjectAccountState};
use crate::error::CandyMachineAffiliatesError;
use crate::utils::validate_client_pda;

#[allow(unused_variables)]
pub fn update_project_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
    affiliate_fee_percentage: f64,
    affiliate_target_in_sol: u8,
    max_affiliate_count: u8,
    title: String,
) -> ProgramResult {
    msg!("Updating project account...");

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let clock = Clock::get().unwrap();

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

    // Validate data

    if title.chars().count() > MAX_PROJECT_TITLE_LENGTH {
        msg!("Project title too long");
        return Err(CandyMachineAffiliatesError::ProjectTitleTooLong.into());
    }

    if max_affiliate_count <= 0 {
        msg!("Invalid project max affiliate count");
        return Err(CandyMachineAffiliatesError::ProjectTitleTooLong.into());
    }

    msg!("Unpacking state account");
    let mut account_state = try_from_slice_unchecked::<ProjectAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Borrowed account data");

    if max_affiliate_count < account_state.data.affiliate_count {
        msg!("Project affiliate count is larger than new max affiliate count");
        return Err(CandyMachineAffiliatesError::ProjectAffiliateCountLargerThanNewMaxAffiliateCount.into());
    }

    account_state.data.affiliate_fee_percentage = affiliate_fee_percentage;
    account_state.data.affiliate_target_in_sol = affiliate_target_in_sol;
    account_state.data.max_affiliate_count = max_affiliate_count;
    account_state.data.title = title;
    account_state.data.updated_at = clock.unix_timestamp;

    msg!("Serializing account");
    account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    Ok(())
}
