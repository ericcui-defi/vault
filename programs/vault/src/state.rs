use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]

// Data stored in the `data` field of the Account structure
pub struct VaultState{
    pub authority: Pubkey,
    pub bump: u8,
    pub total_deposited: u64,
}