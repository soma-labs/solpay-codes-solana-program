use borsh::{
    BorshDeserialize,
    BorshSerialize
};
use solana_program::program_pack::{IsInitialized, Sealed};
use solana_program::pubkey::Pubkey;

pub const PROJECT_ACCOUNT_DATA_VERSION: u8 = 0;
pub const MAX_PROJECT_TITLE_LENGTH: usize = 50;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProjectAccountStateData {
    pub project_owner_pubkey: Pubkey,
    pub candy_machine_id: Pubkey,
    pub affiliate_fee_percentage: f64,
    pub affiliate_target_in_sol: u8,
    pub max_affiliate_count: u8,
    pub affiliate_count: u8,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProjectAccountState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub data_version: u8,
    pub data: ProjectAccountStateData,
}

impl Sealed for ProjectAccountState {}

impl IsInitialized for ProjectAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl ProjectAccountState {
    pub const DISCRIMINATOR: &'static str = "project_account";
    pub const LENGTH: usize = (4 + ProjectAccountState::DISCRIMINATOR.len())
        // is_initialized
        + 1
        // data_version
        + 1
        // project_owner_pubkey
        + 32
        // candy_machine_id
        + 32
        // affiliate_fee_percentage
        + 8
        // affiliate_target_in_sol
        + 1
        // max_affiliate_count
        + 1
        // affiliate_count
        + 1
        // title
        + (4 + 4 * MAX_PROJECT_TITLE_LENGTH)
        // created_at
        + 8
        // updated_at
        + 8
    ;
}
