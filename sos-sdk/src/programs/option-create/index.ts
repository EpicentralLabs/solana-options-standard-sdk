import { PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";
import { Program, AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { createMint, mintTo } from "@solana/program-2022";
import { SOS_PROGRAM_ID } from "./idl"; 
import { MINT_AUTHORITY_SEED } from "./utils";
import { TransactionBuilder } from "@solana/kit";  // Import from Solana Kit

/**
 * Parameters for initializing an option.
 */
export type InitOptionParams = {
  program: Program<SOS_PROGRAM>;            // The Anchor program instance for interacting with the program.
  payer: web3.Signer;                       // The signer who will pay for the transaction fees.
  optionMintKeypair: web3.Keypair;          // Keypair for the option mint (this is the token minting the option).
  quantity: number;                         // Quantity of the option tokens to mint.
  strikePrice: number;                      // Strike price for the option.
  expiration: number;                       // Expiration time (UNIX timestamp) for the option.
  optionPrice: number;                      // Price at which the option can be exercised.
  optionType: number;                       // Type of the option (e.g., 0 for call, 1 for put).
};

/**
 * Initializes a new option on the blockchain, creating necessary accounts and minting tokens.
 * 
 * @param {InitOptionParams} params - The parameters needed for initializing the option.
 * @returns {Promise<{optionState: PublicKey, optionTokenMint: PublicKey, ownerTokenAccount: PublicKey}>} 
 * Returns the created option state, option token mint address, and the owner's token account.
 */
export async function initializeOption({
  program,
  payer,
  optionMintKeypair,
  quantity,
  strikePrice,
  expiration,
  optionPrice,
  optionType,
}: InitOptionParams) {
  // Create a new Keypair for the option state
  const optionStateKeypair = web3.Keypair.generate();

  // Find the mint authority associated with this program
  const [mintAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from(MINT_AUTHORITY_SEED)],
    program.programId
  );

  const optionTokenMint = optionMintKeypair.publicKey;
  const optionState = optionStateKeypair.publicKey;

  // Fetch the associated token account for the payer
  const ownerTokenAccount = await getAssociatedTokenAddress(
    optionTokenMint,
    payer.publicKey
  );

  // Build the transaction using Solana Kit's TransactionBuilder
  const tx = new TransactionBuilder(program.provider.connection)
    .add(
      // Initialize Option - Set up the option mint and state
      program.methods
        .initializeOption(
          new BN(quantity),
          new BN(strikePrice),
          new BN(expiration),
          new BN(optionPrice),
          optionType
        )
        .accounts({
          optionState,
          optionTokenMint,
          mintAuthority,
          ownerTokenAccount,
          payer: payer.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([payer, optionMintKeypair, optionStateKeypair])
    )
    .add(
      // Create Mint for the Option Token using Token Program 2022
      createMint(
        payer,
        mintAuthority,
        optionTokenMint,
        9, // Decimals for the token mint
        program.provider.connection
      )
    )
    .add(
      // Mint Tokens to the owner account
      mintTo(
        payer,
        mintAuthority,
        optionTokenMint,
        ownerTokenAccount,
        quantity
      )
    );

  // Send the transaction
  await tx.build().send();

  return {
    optionState,
    optionTokenMint,
    ownerTokenAccount,
  };
}

/**
 * Exercises an option, allowing the owner to execute the option action.
 * 
 * @param {Program<SOS_PROGRAM>} program - The Anchor program instance for interacting with the SOS program.
 * @param {PublicKey} optionAccount - The public key of the option account to exercise.
 * @param {web3.Signer} owner - The owner of the option who will exercise it.
 * @returns {Promise<void>} A promise that resolves when the transaction has been successfully sent.
 */
export async function exerciseOption(
  program: Program<SOS_PROGRAM>,
  optionAccount: PublicKey,
  owner: web3.Signer
) {
  const tx = new TransactionBuilder(program.provider.connection)
    .add(
      program.methods
        .exerciseOption()
        .accounts({
          optionAccount,
          owner: owner.publicKey,
        })
        .signers([owner])
    );
  
  // Send the transaction
  await tx.build().send();
}

/**
 * Expires an option, effectively invalidating it after the expiration date.
 * 
 * @param {Program<SOS_PROGRAM>} program - The Anchor program instance for interacting with the SOS program.
 * @param {PublicKey} optionAccount - The public key of the option account to expire.
 * @param {web3.Signer} owner - The owner of the option who will expire it.
 * @returns {Promise<void>} A promise that resolves when the transaction has been successfully sent.
 */
export async function expireOption(
  program: Program<SOS_PROGRAM>,
  optionAccount: PublicKey,
  owner: web3.Signer
) {
  const tx = new TransactionBuilder(program.provider.connection)
    .add(
      program.methods
        .expireOption()
        .accounts({
          optionAccount,
          owner: owner.publicKey,
        })
        .signers([owner])
    );
  
  // Send the transaction
  await tx.build().send();
}
