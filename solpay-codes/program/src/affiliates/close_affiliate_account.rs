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
    system_program::ID as SYSTEM_PROGRAM_ID,
};
use borsh::BorshSerialize;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program::invoke;
use crate::affiliates::state::{AFFILIATE_ACCOUNT_DATA_VERSION, AffiliateAccountState};
use crate::error::CandyMachineAffiliatesError;
use crate::{ADMIN_PUBKEY, SOLPAY_TREASURY_PUBKEY};
use crate::projects::state::ProjectAccountState;
use crate::utils::validate_client_pda;

#[allow(unused_variables)]
pub fn close_affiliate_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    affiliate_pubkey: Pubkey,
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
) -> ProgramResult {
    msg!("Closing affiliate account...");

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let project_pda_account = next_account_info(account_info_iter)?;
    let solpay_treasury = next_account_info(account_info_iter)?;

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
            AffiliateAccountState::DISCRIMINATOR.as_ref(),
            affiliate_pubkey.as_ref(),
            project_owner_pubkey.as_ref(),
            candy_machine_id.as_ref(),
        ],
        program_id
    );

    if !client_pda_is_valid {
        msg!("Invalid seeds for PDA");
        return Err(CandyMachineAffiliatesError::InvalidPDA.into());
    }

    let (client_pda_is_valid, bump_seed) = validate_client_pda(
        project_pda_account,
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

    // Validate treasury account
    let solpay_treasury_pubkey = Pubkey::from_str(&SOLPAY_TREASURY_PUBKEY).expect("Pubkey conversion failed");

    if solpay_treasury_pubkey.ne(solpay_treasury.key) {
        msg!("Incorrect treasury account");
        return Err(CandyMachineAffiliatesError::IncorrectTreasuryAccount.into());
    }

    msg!("Unpacking affiliate account state");
    let affiliate_account_state = try_from_slice_unchecked::<AffiliateAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Borrowed affiliate account data");

    msg!("Checking if affiliate account is initialized");
    if !affiliate_account_state.is_initialized() {
        msg!("Affiliate account not initialized");
        return Err(CandyMachineAffiliatesError::UninitializedAccount.into());
    }

    **solpay_treasury.try_borrow_mut_lamports()? += pda_account.lamports();
    **pda_account.try_borrow_mut_lamports()? = 0;
    *pda_account.try_borrow_mut_data()? = &mut [];

    msg!("Affiliate account closed.");

    msg!("Unpacking project state account");
    let mut project_account_state = try_from_slice_unchecked::<ProjectAccountState>(&project_pda_account.data.borrow()).unwrap();
    msg!("Borrowed project account data");

    project_account_state.data.affiliate_count -= 1;

    msg!("Updating project state account");
    project_account_state.serialize(&mut &mut project_pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    Ok(())
}
