use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]

pub struct VaultState{
    pub authority: Pubkey,
    pub bump: u8,
    pub total_deposited: u64,
}