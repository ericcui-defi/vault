use anchor_lang::prelude::*;
use crate::state::VaultState;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub vault: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {

    // Decrease vault lamports
    let vault_account = ctx.accounts.vault.to_account_info();
    **vault_account.try_borrow_mut_lamports()? -= amount;

    // Increase authority lamports
    let authority_account = ctx.accounts.authority.to_account_info();
    **authority_account.try_borrow_mut_lamports()? += amount;

    Ok(())
}