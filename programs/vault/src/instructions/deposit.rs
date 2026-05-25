use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::VaultState;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {

    // Bundling accounts for System Program CPI
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.key(),
        system_program::Transfer{ 
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        }
    );
    system_program::transfer(cpi_context, amount)?;
    let vault = &mut ctx.accounts.vault;
    vault.total_deposited += amount;
    Ok(())
}