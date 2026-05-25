use anchor_lang::prelude::*;
use crate::state::VaultState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub my_account: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let my_account = &mut ctx.accounts.my_account;
    my_account.authority = ctx.accounts.user.key();
    my_account.bump = ctx.bumps.my_account;
    my_account.total_deposited = 0;
    Ok(())
}
