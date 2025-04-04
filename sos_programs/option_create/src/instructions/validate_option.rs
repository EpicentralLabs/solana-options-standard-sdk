use anchor_lang::prelude::*;
use common::types::{
    market_data_types::MarketParams,
    option_data_types::OptionParams,
    token_data_types::TokenParams,
};
use crate::{constants::*, errors::OptionError};

pub fn validate_option_parameters(
    option_params: &OptionParams,
    token_params: &TokenParams,
    market_params: &MarketParams,
) -> Result<()> {
    msg!("Validating expiry timestamp...");
    require!(
        option_params.time_until_expiry > market_params.current_timestamp,
        {
            msg!("Validation failed: Invalid expiry timestamp");
            OptionError::InvalidExpiryTimestamp {
                expected: market_params.current_timestamp,
                actual: option_params.time_until_expiry
            }
        }
    );

    msg!("Validating strike price...");
    require!(
        option_params.strike_price >= MIN_STRIKE_PRICE,
        {
            msg!("Validation failed: Invalid strike price");
            OptionError::InvalidStrikePrice {
                min: MIN_STRIKE_PRICE,
                actual: option_params.strike_price
            }
        }
    );

    // Validate option duration
    let duration = option_params.time_until_expiry - market_params.current_timestamp;
    require!(
        duration >= MIN_OPTION_DURATION && duration <= MAX_OPTION_DURATION,
        OptionError::InvalidDuration
    );

    // Validate volatility
    require!(
        token_params.historical_volatility >= MIN_VOLATILITY 
            && token_params.historical_volatility <= MAX_VOLATILITY,
        OptionError::InvalidVolatility
    );

    // Validate spot price
    require!(
        token_params.spot_price > 0.0,
        OptionError::InvalidSpotPrice
    );

    // Validate risk-free rate
    require!(
        token_params.risk_free_rate >= 0.0,
        OptionError::InvalidRiskFreeRate
    );

    msg!("All validations passed successfully");
    Ok(())
}
