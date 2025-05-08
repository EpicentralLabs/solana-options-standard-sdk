import * as web3 from '@solana/web3.js';
import { Program, AnchorProvider, BN } from '@coral-xyz/anchor';
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { initializeOption, exerciseOption, expireOption, InitOptionParams } from '../index'; 
import { IDL, SOS_PROGRAM } from './idl.json';

import { TransactionBuilder } from '@solana/kit'; 

MINT_AUTHORITY_SEED = "mint-authority"; // Define this 





// Initialize the connection to Solana Devnet
const connection = new web3.Connection(web3.clusterApiUrl('devnet'), 'confirmed');

// Define the payer (the person running the test, should have enough SOL on Devnet)
const payer = web3.Keypair.generate();

// Set up Anchor's provider with the connection and payer
const provider = new AnchorProvider(connection, new web3.Account(payer.secretKey), {
  commitment: 'confirmed',
});

async function run() {

  const program = new Program<SOS_PROGRAM>(IDL, new web3.PublicKey('SOS_PROGRAMPublicKey'), provider);

  // Log payer's address (to check if it's correctly generated)
  console.log('Payer Address:', payer.publicKey.toBase58());

  // 1. Initialize an Option
  const optionMintKeypair = web3.Keypair.generate();
  const optionParams: InitOptionParams = {
    program,
    payer,
    optionMintKeypair,
    quantity: 1000,         // Example quantity
    strikePrice: 100,       // Example strike price (in lamports, adjust as needed)
    expiration: Date.now() + 10000000,  // Example expiration time
    optionPrice: 5,         // Example option price (in lamports)
    optionType: 0,          // Example option type (0 for call, 1 for put)
  };

  console.log('Initializing option...');
  
  // Leverage TransactionBuilder from Solana Kit for the initialization
  const txBuilder = new TransactionBuilder(connection)
    .add(
      // Initialize Option - Solana Token Mint and Option Program interaction
      program.methods
        .initializeOption(
          new BN(optionParams.quantity),
          new BN(optionParams.strikePrice),
          new BN(optionParams.expiration),
          new BN(optionParams.optionPrice),
          optionParams.optionType
        )
        .accounts({
          optionState: web3.Keypair.generate().publicKey,
          optionTokenMint: optionMintKeypair.publicKey,
          mintAuthority: web3.PublicKey.findProgramAddressSync([Buffer.from(MINT_AUTHORITY_SEED)], program.programId)[0],
          ownerTokenAccount: await web3.Token.getAssociatedTokenAddress(
            ASSOCIATED_TOKEN_PROGRAM_ID,
            TOKEN_PROGRAM_ID,
            optionMintKeypair.publicKey,
            payer.publicKey
          ),
          payer: payer.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
          rent: web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([payer, optionMintKeypair]) // Signers for the transaction
    )
    .add(
      // Create Mint for the Option Token using Token Program 2022
      program.provider.connection.getRecentBlockhash().then((blockhash) => {
        return web3.SystemProgram.createAccount({
          fromPubkey: payer.publicKey,
          newAccountPubkey: optionMintKeypair.publicKey,
          lamports: await program.provider.connection.getMinimumBalanceForRentExemption(web3.MintLayout.span),
          space: web3.MintLayout.span,
          programId: TOKEN_PROGRAM_ID,
        });
      })
    )
    .add(
      // Mint Tokens to the owner account
      program.provider.connection.getRecentBlockhash().then((blockhash) => {
        return web3.Token.createMintToInstruction(
          TOKEN_PROGRAM_ID,
          optionMintKeypair.publicKey,
          await web3.Token.getAssociatedTokenAddress(
            ASSOCIATED_TOKEN_PROGRAM_ID,
            TOKEN_PROGRAM_ID,
            optionMintKeypair.publicKey,
            payer.publicKey
          ),
          payer.publicKey,
          [],
          optionParams.quantity
        );
      })
    );

  // Build and send the transaction using Solana Kit
  const transaction = await txBuilder.build();
  const txId = await transaction.send();
  
  console.log('Transaction ID:', txId);

  // 2. Exercise the Option
  console.log('Exercising option...');
  
  // Directly using the `exerciseOption` function
  await exerciseOption(program, optionMintKeypair.publicKey, payer);
  console.log('Option exercised successfully.');

  // 3. Expire the Option
  console.log('Expiring option...');
  
  // Directly using the `expireOption` function
  await expireOption(program, optionMintKeypair.publicKey, payer);
  console.log('Option expired successfully.');
}

// Run the test
run().catch((err) => {
  console.error('Error in test:', err);
});
