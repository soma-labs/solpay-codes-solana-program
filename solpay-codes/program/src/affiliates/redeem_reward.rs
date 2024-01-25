use std::str::FromStr;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::IsInitialized
};
use borsh::BorshSerialize;
use solana_program::native_token::LAMPORTS_PER_SOL;
use crate::affiliates::state::AffiliateAccountState;
use crate::error::CandyMachineAffiliatesError;
use crate::projects::state::ProjectAccountState;
use crate::utils::validate_client_pda;

#[allow(unused_variables)]
pub fn redeem_reward(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey
) -> ProgramResult {
    msg!("Redeeming reward...");

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let affiliate_pda_account = next_account_info(account_info_iter)?;
    let project_pda_account = next_account_info(account_info_iter)?;

    // Validate accounts

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature)
    }

    if affiliate_pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner)
    }

    if project_pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner)
    }

    let (client_affiliate_pda_is_valid, _) = validate_client_pda(
        affiliate_pda_account,
        &[
            AffiliateAccountState::DISCRIMINATOR.as_ref(),
            initializer.key.as_ref(),
            project_owner_pubkey.as_ref(),
            candy_machine_id.as_ref(),
        ],
        program_id
    );

    if !client_affiliate_pda_is_valid {
        msg!("Invalid seeds for affiliate PDA");
        return Err(CandyMachineAffiliatesError::InvalidPDA.into());
    }

    let (client_project_pda_is_valid, _) = validate_client_pda(
        project_pda_account,
        &[
            ProjectAccountState::DISCRIMINATOR.as_ref(),
            project_owner_pubkey.as_ref(),
            candy_machine_id.as_ref(),
        ],
        program_id
    );

    if !client_project_pda_is_valid {
        msg!("Invalid seeds for project PDA");
        return Err(CandyMachineAffiliatesError::InvalidPDA.into());
    }

    msg!("Unpacking affiliate account state");
    let mut affiliate_account_state = try_from_slice_unchecked::<AffiliateAccountState>(&affiliate_pda_account.data.borrow()).unwrap();
    msg!("Borrowed affiliate account data");

    msg!("Checking if affiliate account is initialized");
    if !affiliate_account_state.is_initialized() {
        msg!("Affiliate account not initialized");
        return Err(CandyMachineAffiliatesError::UninitializedAccount.into());
    }

    msg!("Unpacking project account state");
    let project_account_state = try_from_slice_unchecked::<ProjectAccountState>(&project_pda_account.data.borrow()).unwrap();
    msg!("Borrowed project account data");

    msg!("Checking if project account is initialized");
    if !project_account_state.is_initialized() {
        msg!("Project account not initialized");
        return Err(CandyMachineAffiliatesError::UninitializedAccount.into());
    }

    if affiliate_account_state.data.project_owner_pubkey.ne(&project_account_state.data.project_owner_pubkey)
        && affiliate_account_state.data.candy_machine_id.ne(&project_account_state.data.candy_machine_id) {
        msg!("Mismatched accounts when redeeming reward");
        return Err(CandyMachineAffiliatesError::RewardRedeemMismatchedAccounts.into());
    }

    if affiliate_pda_account.lamports() < project_account_state.data.affiliate_target_in_sol as u64 * LAMPORTS_PER_SOL {
        msg!("Affiliate account balance has not reached the threshold");
        return Err(CandyMachineAffiliatesError::AffiliateAccountBalanceNotEnough.into());
    }

    **initializer.try_borrow_mut_lamports()? += project_account_state.data.affiliate_target_in_sol as u64 * LAMPORTS_PER_SOL;
    **affiliate_pda_account.try_borrow_mut_lamports()? -= project_account_state.data.affiliate_target_in_sol as u64 * LAMPORTS_PER_SOL;

    msg!("Reward redeemed: {} SOL.", project_account_state.data.affiliate_target_in_sol);

    affiliate_account_state.data.total_redeemed_amount_in_sol += project_account_state.data.affiliate_target_in_sol as u32;

    msg!("Serializing account");
    affiliate_account_state.serialize(&mut &mut affiliate_pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    Ok(())
}
