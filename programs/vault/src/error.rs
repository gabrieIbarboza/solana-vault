use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only the vault authority can withdraw funds")]
    Unauthorized,
    #[msg("Insufficient funds in the vault")]
    InsufficientFunds,
}

