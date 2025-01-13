// common/tests/black_scholes_model.rs

use common::types::{
    market_data_types::MarketParams,
    option_data_types::{OptionGreeks, OptionParams, OptionType},
    token_data_types::TokenParams,
};
use common::utils::black_scholes_model::{calc_greeks, calc_option_price};

#[test]
fn test_with_custom_parameters() {
    let spot_price = 100.0;
    let strike_price = 100.0;
    let volatility = 0.2;
    let risk_free_rate = 0.05;
    let initial_time_to_expiry = 31_536_000;
    let current_timestamp = 0;

    let option_params = OptionParams {
        option_type: OptionType::LongCall,
        strike_price,
        initial_time_to_expiry,
        creation_price: 0.0,
        greeks: OptionGreeks {
            delta: 0.0,
            gamma: 0.0,
            theta: 0.0,
            vega: 0.0,
            rho: 0.0,
        },
    };

    let token_params = TokenParams {
        spot_price,
        historical_volatility: volatility,
        risk_free_rate,
        timestamp: current_timestamp,
    };

    let market_params = MarketParams {
        usdc_risk_free_rate: risk_free_rate,
        time_in_years: 1.0,
        current_timestamp,
    };

    let price = calc_option_price(&option_params, &token_params, &market_params);
    println!("Calculated Option Price: {}", price);
    assert!(price >= 0.0, "Option price should be >= 0.0");

    let greeks = calc_greeks(&option_params, &token_params, &market_params);
    println!("Calculated Greeks: {:?}", greeks);
    assert!(greeks.delta >= 0.0, "Delta should be >= 0.0 for a LongCall");
}
