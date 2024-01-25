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
use solana_program::clock::Clock;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program::invoke;
use crate::affiliates::state::{AFFILIATE_ACCOUNT_DATA_VERSION, AffiliateAccountState};
use crate::error::CandyMachineAffiliatesError;
use crate::projects::state::ProjectAccountState;
use crate::SOLPAY_TREASURY_PUBKEY;
use crate::utils::validate_client_pda;

const AFFILIATE_REGISTRATION_FEE: u64 = (0.1 * LAMPORTS_PER_SOL as f64) as u64;

#[allow(unused_variables)]
pub fn register_affiliate_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
) -> ProgramResult {
    msg!("Creating affiliate account...");

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let project_pda_account = next_account_info(account_info_iter)?;
    let solpay_treasury = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let clock = Clock::get().unwrap();

    // Validate accounts

    if system_program.key.ne(&SYSTEM_PROGRAM_ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (client_pda_is_valid, bump_seed) = validate_client_pda(
        pda_account,
        &[
            AffiliateAccountState::DISCRIMINATOR.as_ref(),
            initializer.key.as_ref(),
            project_owner_pubkey.as_ref(),
            candy_machine_id.as_ref(),
        ],
        program_id
    );

    if !client_pda_is_valid {
        msg!("Invalid seeds for PDA");
        return Err(CandyMachineAffiliatesError::InvalidPDA.into());
    }

    let (client_pda_is_valid, _) = validate_client_pda(
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

    // Pay affiliate registration fee
    invoke(
        &system_instruction::transfer(
            &initializer.key,
            &solpay_treasury.key,
            AFFILIATE_REGISTRATION_FEE,
        ),
        &[initializer.clone(), solpay_treasury.clone(), system_program.clone()],
    )?;

    // Check if project max affiliates count reached

    msg!("Unpacking project state account");
    let mut project_account_state = try_from_slice_unchecked::<ProjectAccountState>(&project_pda_account.data.borrow()).unwrap();
    msg!("Borrowed project account data");

    if project_account_state.data.affiliate_count == project_account_state.data.max_affiliate_count {
        msg!("Project max affiliate count reached");
        return Err(CandyMachineAffiliatesError::ProjectMaxAffiliateCountReached.into());
    }

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(AffiliateAccountState::LENGTH);

    // Create the affiliate account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            AffiliateAccountState::LENGTH.try_into().unwrap(),
            program_id,
        ),
        &[initializer.clone(), pda_account.clone(), system_program.clone()],
        &[
            &[
                AffiliateAccountState::DISCRIMINATOR.as_ref(),
                initializer.key.as_ref(),
                project_owner_pubkey.as_ref(),
                candy_machine_id.as_ref(),
                &[bump_seed]
            ]
        ],
    )?;

    msg!("PDA created");

    msg!("Unpacking state account");
    let mut account_state = try_from_slice_unchecked::<AffiliateAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Borrowed account data");

    msg!("Checking if affiliate account is already initialized");
    if account_state.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_state.discriminator = AffiliateAccountState::DISCRIMINATOR.to_string();
    account_state.is_initialized = true;
    account_state.data_version = AFFILIATE_ACCOUNT_DATA_VERSION;
    account_state.data.affiliate_pubkey = *initializer.key;
    account_state.data.project_owner_pubkey = project_owner_pubkey;
    account_state.data.candy_machine_id = candy_machine_id;
    account_state.data.total_redeemed_amount_in_sol = 0;
    account_state.data.created_at = clock.unix_timestamp;

    msg!("Serializing account");
    account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    project_account_state.data.affiliate_count += 1;

    msg!("Updating project state account");
    project_account_state.serialize(&mut &mut project_pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    Ok(())
}
