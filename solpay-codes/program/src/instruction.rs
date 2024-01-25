use borsh::{BorshDeserialize};
use solana_program::{program_error::ProgramError};
use solana_program::pubkey::Pubkey;

pub enum CandyMachineAffiliatesInstruction {
    RegisterProject {
        candy_machine_id: Pubkey,
        affiliate_fee_percentage: f64,
        affiliate_target_in_sol: u8,
        max_affiliate_count: u8,
        title: String,
    },
    UpdateProject {
        project_owner_pubkey: Pubkey,
        candy_machine_id: Pubkey,
        affiliate_fee_percentage: f64,
        affiliate_target_in_sol: u8,
        max_affiliate_count: u8,
        title: String,
    },
    CloseProject {
        project_owner_pubkey: Pubkey,
        candy_machine_id: Pubkey,
    },
    RegisterAffiliate {
        project_owner_pubkey: Pubkey,
        candy_machine_id: Pubkey,
    },
    RedeemReward {
        project_owner_pubkey: Pubkey,
        candy_machine_id: Pubkey,
    },
    CloseAffiliateAccount {
        affiliate_pubkey: Pubkey,
        project_owner_pubkey: Pubkey,
        candy_machine_id: Pubkey,
    }
}

#[derive(BorshDeserialize)]
pub struct RegisterProjectPayload {
    candy_machine_id: Pubkey,
    affiliate_fee_percentage: f64,
    affiliate_target_in_sol: u8,
    max_affiliate_count: u8,
    title: String,
}

#[derive(BorshDeserialize)]
pub struct UpdateProjectPayload {
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
    affiliate_fee_percentage: f64,
    affiliate_target_in_sol: u8,
    max_affiliate_count: u8,
    title: String,
}

#[derive(BorshDeserialize)]
pub struct CloseProjectPayload {
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
}

#[derive(BorshDeserialize)]
pub struct RegisterAffiliatePayload {
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
}

#[derive(BorshDeserialize)]
pub struct RedeemRewardPayload {
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
}

#[derive(BorshDeserialize)]
pub struct CloseAffiliatePayload {
    affiliate_pubkey: Pubkey,
    project_owner_pubkey: Pubkey,
    candy_machine_id: Pubkey,
}

impl CandyMachineAffiliatesInstruction {
    // Unpack inbound buffer to associated Instruction
    // The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Split the first byte of data
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload: RegisterProjectPayload = RegisterProjectPayload::try_from_slice(rest).unwrap();

                Self::RegisterProject {
                    candy_machine_id: payload.candy_machine_id,
                    affiliate_fee_percentage: payload.affiliate_fee_percentage,
                    affiliate_target_in_sol: payload.affiliate_target_in_sol,
                    max_affiliate_count: payload.max_affiliate_count,
                    title: payload.title,
                }
            },
            1 => {
                let payload: UpdateProjectPayload = UpdateProjectPayload::try_from_slice(rest).unwrap();

                Self::UpdateProject {
                    project_owner_pubkey: payload.project_owner_pubkey,
                    candy_machine_id: payload.candy_machine_id,
                    affiliate_fee_percentage: payload.affiliate_fee_percentage,
                    affiliate_target_in_sol: payload.affiliate_target_in_sol,
                    max_affiliate_count: payload.max_affiliate_count,
                    title: payload.title,
                }
            },
            2 => {
                let payload: CloseProjectPayload = CloseProjectPayload::try_from_slice(rest).unwrap();

                Self::CloseProject {
                    project_owner_pubkey: payload.project_owner_pubkey,
                    candy_machine_id: payload.candy_machine_id,
                }
            },
            3 => {
                let payload: RegisterAffiliatePayload = RegisterAffiliatePayload::try_from_slice(rest).unwrap();

                Self::RegisterAffiliate {
                    project_owner_pubkey: payload.project_owner_pubkey,
                    candy_machine_id: payload.candy_machine_id,
                }
            },
            4 => {
                let payload: RedeemRewardPayload = RedeemRewardPayload::try_from_slice(rest).unwrap();

                Self::RedeemReward {
                    project_owner_pubkey: payload.project_owner_pubkey,
                    candy_machine_id: payload.candy_machine_id,
                }
            },
            5 => {
                let payload: CloseAffiliatePayload = CloseAffiliatePayload::try_from_slice(rest).unwrap();

                Self::CloseAffiliateAccount {
                    affiliate_pubkey: payload.affiliate_pubkey,
                    project_owner_pubkey: payload.project_owner_pubkey,
                    candy_machine_id: payload.candy_machine_id,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}
