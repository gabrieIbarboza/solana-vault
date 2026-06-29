use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub authority: Pubkey,  // who can withdraw
    pub bump: u8,
}