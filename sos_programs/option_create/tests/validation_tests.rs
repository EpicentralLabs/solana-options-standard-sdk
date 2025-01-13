use anchor_lang::prelude::*;
use option_create::{
    errors::OptionError,
    instructions::validate_option::validate_option_parameters,
};
use common::types::{
    market_data_types::MarketParams,
    option_data_types::{OptionParams, OptionType, OptionGreeks},
    token_data_types::TokenParams,
};

fn setup_valid_params() -> (OptionParams, TokenParams, MarketParams) {
    let current_time = 1700000000;
    
    let option_params = OptionParams {
        option_type: OptionType::LongCall,
        strike_price: 100.0,
        initial_time_to_expiry: current_time + 86400, // 1 day in future
        creation_price: 0.0,
        greeks: OptionGreeks {
            delta: 0.0,
            theta: 0.0,
            gamma: 0.0,
            vega: 0.0,
            rho: 0.0,
        },
    };

    let token_params = TokenParams {
        spot_price: 100.0,
        historical_volatility: 0.2,
        risk_free_rate: 0.05,
        timestamp: current_time,
    };

    let market_params = MarketParams {
        usdc_risk_free_rate: 0.05,
        time_in_years: 1.0,
        current_timestamp: current_time,
    };

    (option_params, token_params, market_params)
}

#[test]
fn test_valid_parameters() {
    let (option_params, token_params, market_params) = setup_valid_params();
    
    let result = validate_option_parameters(&option_params, &token_params, &market_params);
    assert!(result.is_ok(), "Valid parameters should pass validation");
}

#[test]
fn test_invalid_expiry() {
    let (mut option_params, token_params, market_params) = setup_valid_params();
    
    // Set expiry in the past
    option_params.initial_time_to_expiry = market_params.current_timestamp - 1000;
    
    let result = validate_option_parameters(&option_params, &token_params, &market_params);
    assert!(matches!(
        result,
        Err(error) if matches!(
            error.downcast_ref::<OptionError>(),
            Some(OptionError::InvalidExpiryTimestamp { .. })
        )
    ));
}

#[test]
fn test_invalid_strike_price() {
    let (mut option_params, token_params, market_params) = setup_valid_params();
    
    option_params.strike_price = 0.0;
    
    let result = validate_option_parameters(&option_params, &token_params, &market_params);
    assert!(matches!(
        result,
        Err(error) if matches!(
            error.downcast_ref::<OptionError>(),
            Some(OptionError::InvalidStrikePrice { .. })
        )
    ));
}

#[test]
fn test_invalid_volatility() {
    let (option_params, mut token_params, market_params) = setup_valid_params();
    
    token_params.historical_volatility = 1.5; // > 100%
    
    let result = validate_option_parameters(&option_params, &token_params, &market_params);
    assert!(matches!(
        result,
        Err(error) if matches!(
            error.downcast_ref::<OptionError>(),
            Some(OptionError::InvalidVolatility { .. })
        )
    ));
} 