use common::types::{OptionDataTypes, TokenDataTypes, MarketDataTypes};
use common::utils::black_scholes_model::{calc_option_price, calc_greeks};

#[test]
fn test_with_custom_parameters() {
    let spot_price = 100.0; // EDITABLE
    let strike_price = 100.0; // EDITABLE
    let volatility = 0.2; // EDITABLE
    let risk_free_rate = 0.05; // EDITABLE
    let initial_time_to_expiry = 31536000; // 1 year in seconds
    let current_timestamp = 0; // EDITABLE

    run_option_price_test(
        spot_price,
        strike_price,
        volatility,
        risk_free_rate,
        initial_time_to_expiry,
        current_timestamp,
    );

    run_greeks_test(
        spot_price,
        strike_price,
        volatility,
        risk_free_rate,
        initial_time_to_expiry,
        current_timestamp,
    );
}


fn run_option_price_test(
    spot_price: f64,
    strike_price: f64,
    volatility: f64,
    risk_free_rate: f64,
    initial_time_to_expiry: i64,
    current_timestamp: i64,
) {
    let option_params = OptionDataTypes::OptionParams {
        option_type: OptionDataTypes::OptionType::LongCall,
        strike_price,
        initial_time_to_expiry,
        creation_price: 0.0,
        greeks: OptionDataTypes::OptionGreeks {
            delta: 0.0,
            gamma: 0.0,
            theta: 0.0,
            vega: 0.0,
            rho: 0.0,
        },
    };

    let token_params = TokenDataTypes::TokenParams {
        spot_price,
        historical_volatility: volatility,
        risk_free_rate,
        timestamp: current_timestamp,
    };

    let market_params = MarketDataTypes::MarketParams {
        usdc_risk_free_rate: risk_free_rate,
        time_in_years: 1.0, // This can be calculated if needed
        current_timestamp,
    };

    let price = calc_option_price(&option_params, &token_params, &market_params);
    println!("Calculated Option Price: {}", price);
    assert!(price > 0.0, "Option price should be greater than zero");
}

fn run_greeks_test(
    spot_price: f64,
    strike_price: f64,
    volatility: f64,
    risk_free_rate: f64,
    initial_time_to_expiry: i64,
    current_timestamp: i64,
) {
    let option_params = OptionDataTypes::OptionParams {
        option_type: OptionDataTypes::OptionType::LongCall,
        strike_price,
        initial_time_to_expiry,
        creation_price: 0.0,
        greeks: OptionDataTypes::OptionGreeks {
            delta: 0.0,
            gamma: 0.0,
            theta: 0.0,
            vega: 0.0,
            rho: 0.0,
        },
    };

    let token_params = TokenDataTypes::TokenParams {
        spot_price,
        historical_volatility: volatility,
        risk_free_rate,
        timestamp: current_timestamp,
    };

    let market_params = MarketDataTypes::MarketParams {
        usdc_risk_free_rate: risk_free_rate,
        time_in_years: 1.0, // TODO: Add time_in_years as input parameter instead of hardcoding to 1.0
        current_timestamp,
    };

    let greeks = calc_greeks(&option_params, &token_params, &market_params);
    println!("Calculated Greeks: {:?}", greeks);
    assert!(greeks.delta > 0.0, "Delta should be greater than zero for a LongCall option");
    assert!(greeks.gamma > 0.0, "Gamma should be greater than zero");
    assert!(greeks.theta < 0.0, "Theta should be negative");
    assert!(greeks.vega > 0.0, "Vega should be greater than zero");
    assert!(greeks.rho > 0.0, "Rho should be greater than zero for a LongCall option");
}
