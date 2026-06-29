use anchor_lang::prelude::*;

use crate::{state::VaultState, constants::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [STATE_SEED, authority.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    /// The vault is a pure SOL account — it holds no data, just lamports.
    /// We derive it from the authority so each user gets their own vault.
    #[account(
        seeds = [VAULT_SEED, authority.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.vault_state.authority = ctx.accounts.authority.key();
    ctx.accounts.vault_state.bump = ctx.bumps.vault_state;

    msg!("Vault initialized for {}", ctx.accounts.authority.key());
    Ok(())
}