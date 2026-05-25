use anchor_lang::prelude::*;
use anchor_lang::system_program;
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
    let bump = ctx.bumps.vault;
    let seeds: &[&[u8]] = &[b"vault", ctx.accounts.authority.key.as_ref(), &[bump]];
    let signer_seeds = &[seeds];
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.key(),
        system_program::Transfer{
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.authority.to_account_info(),
        },
        signer_seeds,
    );
    system_program::transfer(cpi_context, amount)?;
    let vault = &mut ctx.accounts.vault;
    vault.total_deposited -= amount;
    Ok(())
}