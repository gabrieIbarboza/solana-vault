use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::{state::VaultState, constants::*, error::ErrorCode};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE_SEED, authority.key().as_ref()],
        bump = vault_state.bump,
        has_one = authority,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [VAULT_SEED, authority.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(
        amount <= **ctx.accounts.vault.to_account_info().lamports.borrow(),
        ErrorCode::InsufficientFunds,
    );

    let authority_key = ctx.accounts.authority.key();
    let bump = &[ctx.bumps.vault];
    let signer_seeds: &[&[&[u8]]] = &[&[
        VAULT_SEED,
        authority_key.as_ref(),
        bump,
    ]];

    let cpi_program = ctx.accounts.system_program.key();
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    transfer(cpi_ctx, amount)?;

    msg!("Withdrew {} lamports from vault", amount);
    Ok(())
}