import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Buffer } from "buffer";
import type { Vault } from "../target/types/vault";
import BN from "bn.js";

// ── constants ──────────────────────────────────────────────────────────────────

const STATE_SEED    = Buffer.from("state");
const VAULT_SEED    = Buffer.from("vault");
const DEPOSIT_AMOUNT  = new BN(50_000_000);  // 0.05 SOL in lamports
const WITHDRAW_AMOUNT = new BN(25_000_000);  // 0.025 SOL — half of deposit

// ── main ───────────────────────────────────────────────────────────────────────

async function main() {
  console.log("Script started...");

  // ── setup ───────────────────────────────────────────────────────────────────

  const connection = new anchor.web3.Connection(
    anchor.web3.clusterApiUrl("devnet"),
    "confirmed"
  );

  const wallet = anchor.Wallet.local();
  console.log("Wallet:", wallet.publicKey.toString());

  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;
  console.log("Program ID:", program.programId.toString());

  // ── derive PDAs ─────────────────────────────────────────────────────────────

  const [vaultStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [STATE_SEED, wallet.publicKey.toBuffer()],
    program.programId
  );

  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [VAULT_SEED, wallet.publicKey.toBuffer()],
    program.programId
  );

  console.log("Vault state PDA:", vaultStatePda.toString());
  console.log("Vault PDA:      ", vaultPda.toString());

  // ── initialize ──────────────────────────────────────────────────────────────

  console.log("\nInitializing vault...");
  const initTx = await program.methods
    .initialize()
    .accounts({
        authority:     wallet.publicKey,
    })
    .rpc();
  console.log("Init tx:", `https://explorer.solana.com/tx/${initTx}?cluster=devnet`);

  const stateAfterInit = await program.account.vaultState.fetch(vaultStatePda);
  console.log("Authority:", stateAfterInit.authority.toString());

  // ── deposit ─────────────────────────────────────────────────────────────────

  console.log(`\nDepositing ${DEPOSIT_AMOUNT} lamports (0.05 SOL)...`);
  const depositTx = await program.methods
    .deposit(DEPOSIT_AMOUNT)
    .accounts({
        authority:     wallet.publicKey,
    })
    .rpc();
  console.log("Deposit tx:", `https://explorer.solana.com/tx/${depositTx}?cluster=devnet`);

  const balanceAfterDeposit = await connection.getBalance(vaultPda);
  console.log("Vault balance after deposit:", balanceAfterDeposit, "lamports");

  // ── withdraw ────────────────────────────────────────────────────────────────

  console.log(`\nWithdrawing ${WITHDRAW_AMOUNT} lamports (0.025 SOL — half of deposit)...`);
  const withdrawTx = await program.methods
    .withdraw(WITHDRAW_AMOUNT)
    .accounts({
        authority:     wallet.publicKey,
    })
    .rpc();
  console.log("Withdraw tx:", `https://explorer.solana.com/tx/${withdrawTx}?cluster=devnet`);

  const balanceAfterWithdraw = await connection.getBalance(vaultPda);
  console.log("Vault balance after withdraw:", balanceAfterWithdraw, "lamports");

  // ── summary ─────────────────────────────────────────────────────────────────

  console.log("\n── Summary ────────────────────────────────────────");
  console.log("Vault account:", `https://explorer.solana.com/address/${vaultPda.toString()}?cluster=devnet`);
  console.log("Deposited: ", DEPOSIT_AMOUNT.toString(), "lamports");
  console.log("Withdrawn: ", WITHDRAW_AMOUNT.toString(), "lamports");
  console.log("Remaining: ", balanceAfterWithdraw, "lamports");
}

main().catch(console.error);