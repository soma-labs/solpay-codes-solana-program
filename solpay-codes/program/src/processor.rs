use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use crate::instruction::CandyMachineAffiliatesInstruction;
use crate::affiliates::register_affiliate_account::register_affiliate_account;
use crate::affiliates::redeem_reward::redeem_reward;
use crate::affiliates::close_affiliate_account::close_affiliate_account;
use crate::projects::register_project_account::register_project_account;
use crate::projects::update_project_account::update_project_account;
use crate::projects::close_project_account::close_project_account;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CandyMachineAffiliatesInstruction::unpack(instruction_data)?;

    match instruction {
        CandyMachineAffiliatesInstruction::RegisterProject {
            candy_machine_id,
            affiliate_fee_percentage,
            affiliate_target_in_sol,
            max_affiliate_count,
            title
        } => {
            register_project_account(
                program_id,
                accounts,
                candy_machine_id,
                affiliate_fee_percentage,
                affiliate_target_in_sol,
                max_affiliate_count,
                title
            )
        }
        CandyMachineAffiliatesInstruction::UpdateProject {
            project_owner_pubkey,
            candy_machine_id,
            affiliate_fee_percentage,
            affiliate_target_in_sol,
            max_affiliate_count,
            title,
        } => {
            update_project_account(
                program_id,
                accounts,
                project_owner_pubkey,
                candy_machine_id,
                affiliate_fee_percentage,
                affiliate_target_in_sol,
                max_affiliate_count,
                title,
            )
        }
        CandyMachineAffiliatesInstruction::CloseProject {
            project_owner_pubkey,
            candy_machine_id
        } => {
            close_project_account(
                program_id,
                accounts,
                project_owner_pubkey,
                candy_machine_id
            )
        }
        CandyMachineAffiliatesInstruction::RegisterAffiliate {
            project_owner_pubkey,
            candy_machine_id,
        } => {
            register_affiliate_account(
                program_id,
                accounts,
                project_owner_pubkey,
                candy_machine_id,
            )
        }
        CandyMachineAffiliatesInstruction::RedeemReward {
            project_owner_pubkey,
            candy_machine_id,
        } => {
            redeem_reward(
                program_id,
                accounts,
                project_owner_pubkey,
                candy_machine_id
            )
        },
        CandyMachineAffiliatesInstruction::CloseAffiliateAccount {
            affiliate_pubkey,
            project_owner_pubkey,
            candy_machine_id,
        } => {
            close_affiliate_account(
                program_id,
                accounts,
                affiliate_pubkey,
                project_owner_pubkey,
                candy_machine_id
            )
        }
    }
}
