use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

// Derive PDA and check that it matches client
pub fn validate_client_pda(client_pda: &AccountInfo, seeds: &[&[u8]], program_id: &Pubkey) -> (bool, u8) {
    let (pda, bump_seed) = Pubkey::find_program_address(
        &seeds,
        program_id,
    );

    (pda == *client_pda.key, bump_seed)
}
