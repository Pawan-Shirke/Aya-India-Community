import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { IdentityLifecycleVault } from "../target/types/identity_lifecycle_vault";

anchor.setProvider(anchor.AnchorProvider.env());

const provider = anchor.getProvider() as anchor.AnchorProvider;
const program = anchor.workspace
  .IdentityLifecycleVault as Program<IdentityLifecycleVault>;

// ---------------------------------------------
// PDA Helper
// ---------------------------------------------
export const getIdentityPda = async (
  owner: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [Buffer.from("identity"), owner.toBuffer()],
    program.programId
  );
};

// ---------------------------------------------
// Initialize Identity Vault
// ---------------------------------------------
export const initializeIdentity = async () => {
  const owner = provider.wallet.publicKey;
  const [identityPda] = await getIdentityPda(owner);

  await program.methods
    .initializeIdentity()
    .accounts({
      identityVault: identityPda,
      owner: owner,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("✅ Identity vault initialized:", identityPda.toBase58());
};

// ---------------------------------------------
// Record Biometric Update
// ---------------------------------------------
export const biometricUpdate = async () => {
  const owner = provider.wallet.publicKey;
  const [identityPda] = await getIdentityPda(owner);

  await program.methods
    .biometricUpdate()
    .accounts({
      identityVault: identityPda,
      owner: owner,
    })
    .rpc();

  console.log("🧬 Biometric update recorded");
};

// ---------------------------------------------
// Record Demographic Update
// ---------------------------------------------
export const demographicUpdate = async () => {
  const owner = provider.wallet.publicKey;
  const [identityPda] = await getIdentityPda(owner);

  await program.methods
    .demographicUpdate()
    .accounts({
      identityVault: identityPda,
      owner: owner,
    })
    .rpc();

  console.log("📄 Demographic update recorded");
};

// ---------------------------------------------
// Fetch Identity Vault State
// ---------------------------------------------
export const fetchIdentityVault = async () => {
  const owner = provider.wallet.publicKey;
  const [identityPda] = await getIdentityPda(owner);

  const vault = await program.account.identityVault.fetch(identityPda);

  console.log("🔍 Identity Vault State:");
  console.log({
    owner: vault.owner.toBase58(),
    createdAt: new Date(vault.createdAt.toNumber() * 1000),
    lastUpdate: new Date(vault.lastUpdate.toNumber() * 1000),
    updateCount: vault.updateCount.toNumber(),
    lifecycleScore: vault.lifecycleScore.toNumber(),
  });
};

// ---------------------------------------------
// Close Identity Vault
// ---------------------------------------------
export const closeIdentity = async () => {
  const owner = provider.wallet.publicKey;
  const [identityPda] = await getIdentityPda(owner);

  await program.methods
    .closeIdentity()
    .accounts({
      identityVault: identityPda,
      owner: owner,
    })
    .rpc();

  console.log("🗑️ Identity vault closed and rent refunded");
};
