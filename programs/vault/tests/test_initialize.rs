mod helpers;
use helpers::*;
use solana_signer::Signer;

/// ✅ Positive: vault is created with the correct authority.
#[test]
fn test_initialize_success() {
    let (mut svm, user, _stranger) = setup();

    let result = send(&mut svm, initialize_ix(&user), &user);
    assert!(result.is_ok(), "Initialize should succeed");

    let state = get_vault_state(&svm, &user.pubkey());
    assert_eq!(state.authority, user.pubkey(), "Authority should be the user");
}

/// ❌ Negative: initializing the same vault twice should fail
///    because the vault_state account already exists.
#[test]
fn test_initialize_twice_fails() {
    let (mut svm, user, _stranger) = setup();

    send(&mut svm, initialize_ix(&user), &user).unwrap();

    let result = send(&mut svm, initialize_ix(&user), &user);
    assert!(result.is_err(), "Second initialize should fail — account already exists");
}