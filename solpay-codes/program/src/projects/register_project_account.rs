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
    system_program::ID as SYSTEM_PROGRAM_ID
};
use borsh::BorshSerialize;
use solana_program::clock::Clock;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program::invoke;
use crate::projects::state::{MAX_PROJECT_TITLE_LENGTH, PROJECT_ACCOUNT_DATA_VERSION, ProjectAccountState};
use crate::error::CandyMachineAffiliatesError;
use crate::utils::validate_client_pda;

#[allow(unused_variables)]
pub fn register_project_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    candy_machine_id: Pubkey,
    affiliate_fee_percentage: f64,
    affiliate_target_in_sol: u8,
    max_affiliate_count: u8,
    title: String,
) -> ProgramResult {
    msg!("Creating project account...");

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
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
            ProjectAccountState::DISCRIMINATOR.as_ref(),
            initializer.key.as_ref(),
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

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(ProjectAccountState::LENGTH);

    // Create the project account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            ProjectAccountState::LENGTH.try_into().unwrap(),
            program_id,
        ),
        &[initializer.clone(), pda_account.clone(), system_program.clone()],
        &[
            &[
                ProjectAccountState::DISCRIMINATOR.as_ref(),
                initializer.key.as_ref(),
                candy_machine_id.as_ref(),
                &[bump_seed]
            ]
        ],
    )?;

    msg!("PDA created");

    msg!("Unpacking state account");
    let mut account_state = try_from_slice_unchecked::<ProjectAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Borrowed account data");

    msg!("Checking if project account is already initialized");
    if account_state.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_state.discriminator = ProjectAccountState::DISCRIMINATOR.to_string();
    account_state.is_initialized = true;
    account_state.data_version = PROJECT_ACCOUNT_DATA_VERSION;
    account_state.data.project_owner_pubkey = *initializer.key;
    account_state.data.candy_machine_id = candy_machine_id;
    account_state.data.affiliate_fee_percentage = affiliate_fee_percentage;
    account_state.data.affiliate_target_in_sol = affiliate_target_in_sol;
    account_state.data.max_affiliate_count = max_affiliate_count;
    account_state.data.affiliate_count = 0;
    account_state.data.title = title;
    account_state.data.created_at = clock.unix_timestamp;
    account_state.data.updated_at = clock.unix_timestamp;

    msg!("Serializing account");
    account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    Ok(())
}
