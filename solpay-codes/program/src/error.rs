use solana_program::{program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CandyMachineAffiliatesError {
    // 0
    #[error("Action not allowed")]
    ActionNotAllowed,
    // 1
    #[error("Account not initialized yet")]
    UninitializedAccount,
    // 2
    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,
    // 3
    #[error("Input data exceeds max length")]
    InvalidDataLength,
    // 4
    #[error("Amount overflow")]
    AmountOverflow,
    // 5
    #[error("Project title too long")]
    ProjectTitleTooLong,
    // 6
    #[error("Invalid project max affiliate count")]
    InvalidProjectMaxAffiliateCount,
    // 7
    #[error("Project max affiliate count reached")]
    ProjectMaxAffiliateCountReached,
    // 8
    #[error("Project affiliate count is larger than new max affiliate count")]
    ProjectAffiliateCountLargerThanNewMaxAffiliateCount,
    // 9
    #[error("Incorrect treasury account")]
    IncorrectTreasuryAccount,
    // 10
    #[error("Mismatched accounts when redeeming reward")]
    RewardRedeemMismatchedAccounts,
    // 11
    #[error("Affiliate account balance has not reached the redeem threshold")]
    AffiliateAccountBalanceNotEnough,
}

impl From<CandyMachineAffiliatesError> for ProgramError {
    fn from(e: CandyMachineAffiliatesError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
