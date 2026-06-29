mod helpers;
use helpers::*;
use anchor_lang::{InstructionData, ToAccountMetas};
use solana_signer::Signer;
use anchor_lang::solana_program::instruction::Instruction;

/// ✅ Positive: user can withdraw SOL they previously deposited.
#[test]
fn test_withdraw_success() {
    let (mut svm, user, _stranger) = setup();

    send(&mut svm, initialize_ix(&user), &user).unwrap();
    send(&mut svm, deposit_ix(&user, DEPOSIT_AMOUNT), &user).unwrap();

    let result = send(&mut svm, withdraw_ix(&user, WITHDRAW_AMOUNT), &user);
    assert!(result.is_ok(), "Withdraw should succeed");

    let vault_balance = get_vault_balance(&svm, &user.pubkey());
    assert_eq!(
        vault_balance,
        DEPOSIT_AMOUNT - WITHDRAW_AMOUNT,
        "Vault should hold the remaining balance after withdrawal"
    );
}

/// ❌ Negative: withdrawing without initializing first should fail.
#[test]
fn test_withdraw_without_initialize_fails() {
    let (mut svm, user, _stranger) = setup();

    let result = send(&mut svm, withdraw_ix(&user, WITHDRAW_AMOUNT), &user);
    assert!(result.is_err(), "Withdraw should fail — vault not initialized");
}

/// ❌ Negative: withdrawing more than the vault holds should fail
///    with our custom InsufficientFunds error.
#[test]
fn test_withdraw_excessive_amount_fails() {
    let (mut svm, user, _stranger) = setup();

    send(&mut svm, initialize_ix(&user), &user).unwrap();
    send(&mut svm, deposit_ix(&user, SMALL_DEPOSIT), &user).unwrap();

    // Try to withdraw 5 SOL when only 0.5 SOL is in the vault
    let result = send(&mut svm, withdraw_ix(&user, EXCESSIVE_WITHDRAW), &user);
    assert!(result.is_err(), "Withdraw should fail — amount exceeds vault balance");

    // Verify the specific error is InsufficientFunds
    let err = result.unwrap_err();
    let logs = err.meta.logs.join(" ");
    assert!(
        logs.contains("InsufficientFunds"),
        "Expected InsufficientFunds error, got logs: {}", logs
    );

    // Vault balance must remain untouched
    let vault_balance = get_vault_balance(&svm, &user.pubkey());
    assert_eq!(
        vault_balance,
        SMALL_DEPOSIT,
        "Vault balance should be unchanged after failed withdraw"
    );
}

/// ❌ Negative: a stranger cannot withdraw from someone else's vault.
#[test]
fn test_withdraw_wrong_authority_fails() {
    let (mut svm, user, stranger) = setup();

    send(&mut svm, initialize_ix(&user), &user).unwrap();
    send(&mut svm, deposit_ix(&user, DEPOSIT_AMOUNT), &user).unwrap();

    // Stranger references user's vault but signs with their own keypair
    let ix = Instruction::new_with_bytes(
        vault::id(),
        &vault::instruction::Withdraw { amount: WITHDRAW_AMOUNT }.data(),
        vault::accounts::Withdraw {
            authority:      stranger.pubkey(), // ← wrong authority
            vault_state:    derive_vault_state(&user.pubkey()), // ← user's vault
            vault:          derive_vault(&user.pubkey()),
            system_program: anchor_lang::system_program::ID,
        }.to_account_metas(None),
    );

    let result = send(&mut svm, ix, &stranger);
    assert!(result.is_err(), "Withdraw with wrong authority should fail");

    // Vault must remain untouched
    let vault_balance = get_vault_balance(&svm, &user.pubkey());
    assert_eq!(
        vault_balance,
        DEPOSIT_AMOUNT,
        "Vault balance should be unchanged after failed withdraw"
    );
}