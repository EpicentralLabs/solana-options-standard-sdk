use common::types::{
    market_data_types::MarketParams,
    option_data_types::{OptionParams, OptionType, OptionGreeks},
    token_data_types::TokenParams,
};
use common::utils::black_scholes_model::{calc_greeks, calc_option_price};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_option_price() {
        // Initialize your test data
        let option_params = OptionParams {
            option_type: OptionType::LongCall, // CAN CHANGE | Call option | Put Option: `OptionType::LongPut`
            strike_price: 300.0, // CAN CHANGE |
            time_until_expiry: 15_768_000, // CAN CHANGE| in seconds
            creation_price: 0.0, // CAN CHANGE |in USD
            greeks: OptionGreeks {
                delta: 0.0, 
                gamma: 0.0,
                theta: 0.0,
                vega: 0.0,
                rho: 0.0,
            },
        };

        let token_params = TokenParams {
            spot_price: 300.0, // CAN CHANGE |
            historical_volatility: 0.2, // CAN CHANGE |
            risk_free_rate: 0.05, // CAN CHANGE |
            timestamp: 0, // CAN CHANGE |
        };

        let market_params = MarketParams {
            usdc_risk_free_rate: 0.0,
            time_in_years: 0.0,
            current_timestamp: 0,
        };

        let price = calc_option_price(&option_params, &token_params, &market_params);
        println!("Calculated Option Price: {}", price);
        assert!(price > 0.0, "Option price should be greater than 0.0");
    }

    #[test]
    fn test_calc_greeks() {
        // Initialize your test data
        let option_params = OptionParams {
            option_type: OptionType::LongCall,
            strike_price: 100.0,
            time_until_expiry: 31_536_000,
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
            spot_price: 100.0,
            historical_volatility: 0.2,
            risk_free_rate: 0.05,
            timestamp: 0,
        };

        let market_params = MarketParams {
            usdc_risk_free_rate: 0.05,
            time_in_years: 1.0,
            current_timestamp: 0,
        };

        let greeks = calc_greeks(&option_params, &token_params, &market_params);
        println!("Calculated Greeks: {:?}", greeks);
        assert!(greeks.delta >= 0.0, "Delta should be >= 0.0 for a LongCall");
    }

    // TO RUN THE TEST: 
    // cargo test -p common --test black_scholes_test -- --nocapture
    
}
