import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { OptionCreate } from '../target/types/option_create';
import { expect } from 'chai';
import { PublicKey, Keypair } from '@solana/web3.js';

describe('Option Create Program Tests', () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.OptionCreate as Program<OptionCreate>;
    
    // Helper function to create base parameters
    const createValidParams = () => {
        const now = Math.floor(Date.now() / 1000);
        return {
            optionInput: {
                optionType: { longCall: {} },
                strikePrice: 100.0,
                expiryTimestamp: now + 86400, // 1 day in future
            },
            tokenInput: {
                spotPrice: 100.0,
                volatility: 0.2,
                riskFreeRate: 0.05,
            },
            marketInput: {
                usdcRiskFreeRate: 0.05,
                timeInYears: 1.0,
            },
        };
    };

    it('successfully creates option with valid parameters', async () => {
        const params = createValidParams();
        const optionAccount = Keypair.generate();

        const tx = await program.methods
            .createOption(params.optionInput, params.tokenInput, params.marketInput)
            .accounts({
                optionAccount: optionAccount.publicKey,
                authority: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([optionAccount])
            .rpc();

        // Fetch the created account
        const account = await program.account.optionAccount.fetch(
            optionAccount.publicKey
        );

        // Verify account data
        expect(account.strikePrice).to.equal(params.optionInput.strikePrice);
        expect(account.status.open).to.be.true;
    });

    it('fails with expired timestamp', async () => {
        const params = createValidParams();
        params.optionInput.expiryTimestamp = Math.floor(Date.now() / 1000) - 1000;

        try {
            await program.methods
                .createOption(params.optionInput, params.tokenInput, params.marketInput)
                .accounts({
                    optionAccount: Keypair.generate().publicKey,
                    authority: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            expect.fail('Should have thrown an error');
        } catch (error) {
            expect(error.error.errorCode.code).to.equal('InvalidExpiryTimestamp');
        }
    });

    it('fails with invalid volatility', async () => {
        const params = createValidParams();
        params.tokenInput.volatility = 1.5; // 150% volatility

        try {
            await program.methods
                .createOption(params.optionInput, params.tokenInput, params.marketInput)
                .accounts({
                    optionAccount: Keypair.generate().publicKey,
                    authority: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            expect.fail('Should have thrown an error');
        } catch (error) {
            expect(error.error.errorCode.code).to.equal('InvalidVolatility');
        }
    });

    it('validates market conditions', async () => {
        const params = createValidParams();
        // Test extreme market conditions
        params.tokenInput.spotPrice = 1000.0;
        params.optionInput.strikePrice = 100.0; // 90% deviation

        try {
            await program.methods
                .createOption(params.optionInput, params.tokenInput, params.marketInput)
                .accounts({
                    optionAccount: Keypair.generate().publicKey,
                    authority: provider.wallet.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            expect.fail('Should have thrown an error');
        } catch (error) {
            expect(error.error.errorCode.code).to.equal('ExtremeStrikePrice');
        }
    });
});