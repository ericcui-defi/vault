import { AnchorProvider, Program, BN, web3 } from "@anchor-lang/core";
import NodeWallet from "@anchor-lang/core/dist/cjs/nodewallet";
import idl from "../target/idl/vault.json";
import { Keypair, Connection, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import fs from "fs";
import os from "os";
import path from "path";

// Load your local wallet keypair
const keypairPath = path.join(os.homedir(), ".config/solana/id.json");
const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
const wallet = new NodeWallet(Keypair.fromSecretKey(Uint8Array.from(keypairData)));

// Connect to devnet
const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const provider = new AnchorProvider(connection, wallet, { commitment: "confirmed" });

const programId = new PublicKey("6bC76eicZeLaipDf9X7wMmxu8pNvmPbaHtnS7HM3kd6X");
const program = new Program(idl as any, provider);

// Derive the vault PDA
const [vaultPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), wallet.publicKey.toBuffer()],
  programId
);

async function initialize() {
  console.log("Initializing vault...");
  const tx = await program.methods
    .initialize()
    .accounts({
      user: wallet.publicKey,
      myAccount: vaultPda,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();
  console.log("Vault initialized! Tx:", tx);
}

async function deposit(amount: number) {
  console.log(`Depositing ${amount / LAMPORTS_PER_SOL} SOL...`);
  const tx = await program.methods
    .deposit(new BN(amount))
    .accounts({
      user: wallet.publicKey,
      vault: vaultPda,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();
  console.log("Deposit successful! Tx:", tx);
}

async function withdraw(amount: number) {
  console.log(`Withdrawing ${amount / LAMPORTS_PER_SOL} SOL...`);
  const tx = await program.methods
    .withdraw(new BN(amount))
    .accounts({
      authority: wallet.publicKey,
      vault: vaultPda,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();
  console.log("Withdrawal successful! Tx:", tx);
}

async function getBalance() {
  const balance = await connection.getBalance(vaultPda);
  console.log(`Vault balance: ${balance / LAMPORTS_PER_SOL} SOL (${balance} lamports)`);
}

// Parse CLI arguments
const command = process.argv[2];
const amount = process.argv[3] ? parseFloat(process.argv[3]) * LAMPORTS_PER_SOL : 0;

(async () => {
  try {
    switch (command) {
      case "initialize":
        await initialize();
        break;
      case "deposit":
        if (!amount) { console.log("Usage: deposit <amount_in_sol>"); break; }
        await deposit(amount);
        break;
      case "withdraw":
        if (!amount) { console.log("Usage: withdraw <amount_in_sol>"); break; }
        await withdraw(amount);
        break;
      case "balance":
        await getBalance();
        break;
      default:
        console.log("Commands: initialize | deposit <sol> | withdraw <sol> | balance");
    }
  } catch (err) {
    console.error("Error:", err);
  }
})();
