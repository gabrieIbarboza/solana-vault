mod helpers;
use helpers::*;
use anchor_lang::{InstructionData, ToAccountMetas};
use solana_signer::Signer;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::instruction::Instruction;

/// ✅ Positive: user can deposit SOL into their vault.
#[test]
fn test_deposit_success() {
    let (mut svm, user, _stranger) = setup();

    send(&mut svm, initialize_ix(&user), &user).unwrap();

    let result = send(&mut svm, deposit_ix(&user, DEPOSIT_AMOUNT), &user);
    assert!(result.is_ok(), "Deposit should succeed");

    let vault_balance = get_vault_balance(&svm, &user.pubkey());
    assert_eq!(vault_balance, DEPOSIT_AMOUNT, "Vault should hold the deposited amount");
}

/// ❌ Negative: depositing without initializing first should fail
///    because vault_state doesn't exist yet.
#[test]
fn test_deposit_without_initialize_fails() {
    let (mut svm, user, _stranger) = setup();

    let result = send(&mut svm, deposit_ix(&user, DEPOSIT_AMOUNT), &user);
    assert!(result.is_err(), "Deposit should fail — vault not initialized");
}

/// ❌ Negative: a stranger cannot deposit into someone else's vault
///    because has_one = authority will reject the mismatch.
#[test]
fn test_deposit_wrong_authority_fails() {
    let (mut svm, user, stranger) = setup();

    send(&mut svm, initialize_ix(&user), &user).unwrap();

    // Stranger references user's vault_state but signs with their own keypair —
    // has_one = authority will catch this mismatch
    let ix = Instruction::new_with_bytes(
        vault::id(),
        &vault::instruction::Deposit { amount: DEPOSIT_AMOUNT }.data(),
        vault::accounts::Deposit {
            authority:      stranger.pubkey(), // ← wrong authority
            vault_state:    derive_vault_state(&user.pubkey()), // ← user's vault
            vault:          derive_vault(&user.pubkey()),
            system_program: system_program::ID,
        }.to_account_metas(None),
    );

    let result = send(&mut svm, ix, &stranger);
    assert!(result.is_err(), "Deposit with wrong authority should fail");
}