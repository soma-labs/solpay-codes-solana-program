use borsh::{
    BorshDeserialize,
    BorshSerialize
};
use solana_program::program_pack::{IsInitialized, Sealed};
use solana_program::pubkey::Pubkey;

pub const AFFILIATE_ACCOUNT_DATA_VERSION: u8 = 0;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AffiliateAccountStateData {
    pub affiliate_pubkey: Pubkey,
    pub project_owner_pubkey: Pubkey,
    pub candy_machine_id: Pubkey,
    pub total_redeemed_amount_in_sol: u32,
    pub created_at: i64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AffiliateAccountState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub data_version: u8,
    pub data: AffiliateAccountStateData,
}

impl Sealed for AffiliateAccountState {}

impl IsInitialized for AffiliateAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl AffiliateAccountState {
    pub const DISCRIMINATOR: &'static str = "affiliate_account";
    pub const LENGTH: usize = (4 + AffiliateAccountState::DISCRIMINATOR.len())
        // is_initialized
        + 1
        // data_version
        + 1
        // affiliate_pubkey
        + 32
        // project_owner_pubkey
        + 32
        // candy_machine_id
        + 32
        // mint_count
        + 4
        // created_at
        + 8
    ;
}
