mod constants;
mod errors;
mod instructions;
mod state;

use constants::*;

use anchor_lang::prelude::*;
use common::{
    types::{
        market_data_types::MarketParams,
        option_data_types::{OptionGreeks, OptionParams, OptionStatus, OptionType},
        token_data_types::TokenParams,
    },
    utils::black_scholes_model::{calc_greeks, calc_option_price},
};

declare_id!("programID");

#[program]
pub mod option_create {
    use super::*;

    pub fn create_option(
        ctx: Context<CreateOption>,
        option_input: OptionInput,
        token_input: TokenInput,
        market_input: MarketInput,
    ) -> Result<()> {
        msg!("Starting option creation for authority: {}", ctx.accounts.authority.key());
        
        // Log input parameters
        msg!("Option parameters - Strike: {}, Expiry: {}", 
            option_input.strike_price, 
            option_input.expiry_timestamp
        );
        
        // Convert inputs to our common library types
        let option_params = OptionParams {
            option_type: option_input.option_type,
            strike_price: option_input.strike_price,
            initial_time_to_expiry: option_input.expiry_timestamp,
            creation_price: 0.0, // Will be calculated
            greeks: OptionGreeks {
                delta: 0.0,
                theta: 0.0,
                gamma: 0.0,
                vega: 0.0,
                rho: 0.0,
            },
        };

        let token_params = TokenParams {
            spot_price: token_input.spot_price,
            historical_volatility: token_input.volatility,
            risk_free_rate: token_input.risk_free_rate,
            timestamp: Clock::get()?.unix_timestamp,
        };

        let market_params = MarketParams {
            usdc_risk_free_rate: market_input.usdc_risk_free_rate,
            time_in_years: market_input.time_in_years,
            current_timestamp: Clock::get()?.unix_timestamp,
        };

        msg!("Validating option parameters...");
        instructions::validate_option::validate_option_parameters(
            &option_params,
            &token_params,
            &market_params,
        )?;
        msg!("Option parameters validated successfully");

        msg!("Calculating option price and greeks...");
        let creation_price = calc_option_price(&option_params, &token_params, &market_params);
        let greeks = calc_greeks(&option_params, &token_params, &market_params);
        msg!("Calculated option price: {}", creation_price);
        msg!("Greeks - Delta: {}, Gamma: {}, Theta: {}, Vega: {}, Rho: {}", 
            greeks.delta, greeks.gamma, greeks.theta, greeks.vega, greeks.rho
        );

        // Store the validated and calculated data
        msg!("Storing option data on-chain...");
        let option_account = &mut ctx.accounts.option_account;
        option_account.owner = ctx.accounts.authority.key();
        option_account.option_type = option_input.option_type;
        option_account.strike_price = option_input.strike_price;
        option_account.expiry_timestamp = option_input.expiry_timestamp;
        option_account.creation_price = creation_price;
        option_account.greeks = greeks;
        option_account.status = OptionStatus::Open;
        option_account.spot_price_at_creation = token_input.spot_price;

        // Log compute units at the end (helpful for optimization)
        sol_log_compute_units();
        msg!("Option creation completed successfully");
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOption<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + OptionAccount::SIZE
    )]
    pub option_account: Account<'info, OptionAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OptionInput {
    pub option_type: OptionType,
    pub strike_price: f64,
    pub expiry_timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TokenInput {
    pub spot_price: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MarketInput {
    pub usdc_risk_free_rate: f64,
    pub time_in_years: f64,
}
