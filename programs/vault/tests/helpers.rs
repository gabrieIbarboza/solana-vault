use anchor_lang::{
    prelude::Pubkey,
    solana_program::{instruction::Instruction, system_program},
    AccountDeserialize, InstructionData, ToAccountMetas,
};
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

// ── constants ─────────────────────────────────────────────────────────────────

pub const AIRDROP_AMOUNT:     u64 = 10_000_000_000; // 10 SOL
pub const DEPOSIT_AMOUNT:     u64 =  1_000_000_000; // 1 SOL
pub const SMALL_DEPOSIT:      u64 =    500_000_000; // 0.5 SOL
pub const WITHDRAW_AMOUNT:    u64 =    500_000_000; // 0.5 SOL
pub const EXCESSIVE_WITHDRAW: u64 =  5_000_000_000; // 5 SOL — more than vault holds

pub const STATE_SEED: &[u8] = b"state";
pub const VAULT_SEED: &[u8] = b"vault";

// ── setup ─────────────────────────────────────────────────────────────────────

/// Boots a local VM with the vault program loaded.
/// Returns the VM, an authorized user, and a stranger (for negative tests).
pub fn setup() -> (LiteSVM, Keypair, Keypair) {
    let user     = Keypair::new();
    let stranger = Keypair::new();

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(env!("CARGO_TARGET_TMPDIR"), "/../deploy/vault.so"));
    svm.add_program(vault::id(), bytes).unwrap();

    svm.airdrop(&user.pubkey(),     AIRDROP_AMOUNT).unwrap();
    svm.airdrop(&stranger.pubkey(), AIRDROP_AMOUNT).unwrap();

    (svm, user, stranger)
}

// ── PDA derivation helpers ────────────────────────────────────────────────────

pub fn derive_vault_state(authority: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[STATE_SEED, authority.as_ref()],
        &vault::id(),
    ).0
}

pub fn derive_vault(authority: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[VAULT_SEED, authority.as_ref()],
        &vault::id(),
    ).0
}

// ── instruction builders ──────────────────────────────────────────────────────

pub fn initialize_ix(authority: &Keypair) -> Instruction {
    Instruction::new_with_bytes(
        vault::id(),
        &vault::instruction::Initialize {}.data(),
        vault::accounts::Initialize {
            authority:      authority.pubkey(),
            vault_state:    derive_vault_state(&authority.pubkey()),
            vault:          derive_vault(&authority.pubkey()),
            system_program: system_program::ID,
        }.to_account_metas(None),
    )
}

pub fn deposit_ix(authority: &Keypair, amount: u64) -> Instruction {
    Instruction::new_with_bytes(
        vault::id(),
        &vault::instruction::Deposit { amount }.data(),
        vault::accounts::Deposit {
            authority:      authority.pubkey(),
            vault_state:    derive_vault_state(&authority.pubkey()),
            vault:          derive_vault(&authority.pubkey()),
            system_program: system_program::ID,
        }.to_account_metas(None),
    )
}

pub fn withdraw_ix(authority: &Keypair, amount: u64) -> Instruction {
    Instruction::new_with_bytes(
        vault::id(),
        &vault::instruction::Withdraw { amount }.data(),
        vault::accounts::Withdraw {
            authority:      authority.pubkey(),
            vault_state:    derive_vault_state(&authority.pubkey()),
            vault:          derive_vault(&authority.pubkey()),
            system_program: system_program::ID,
        }.to_account_metas(None),
    )
}

// ── transaction sender ────────────────────────────────────────────────────────

pub fn send(
    svm: &mut LiteSVM,
    ix: Instruction,
    signer: &Keypair,
) -> Result<litesvm::types::TransactionMetadata, litesvm::types::FailedTransactionMetadata> {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&signer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[signer]).unwrap();
    svm.send_transaction(tx)
}

// ── state readers ─────────────────────────────────────────────────────────────

pub fn get_vault_state(svm: &LiteSVM, authority: &Pubkey) -> vault::state::VaultState {
    let pda     = derive_vault_state(authority);
    let account = svm.get_account(&pda).unwrap();
    let mut data: &[u8] = &account.data;
    vault::state::VaultState::try_deserialize(&mut data).unwrap()
}

pub fn get_vault_balance(svm: &LiteSVM, authority: &Pubkey) -> u64 {
    let pda = derive_vault(authority);
    svm.get_account(&pda)
        .map(|a| a.lamports)
        .unwrap_or(0)
}