use common::types::{
    market_data_types::MarketParams,
    option_data_types::{OptionParams, OptionType, OptionGreeks},
    token_data_types::TokenParams,
};
use common::utils::black_scholes_model::{calc_greeks, calc_option_price};

#[cfg(test)]
mod tests {
    use super::*;

    // Common test parameters | CAN CHANGE CONSTANTS FOR TESTING
    const OPTION_TYPE: OptionType = OptionType::LongCall;
    const STRIKE_PRICE: f64 = 100.0;
    const TIME_UNTIL_EXPIRY: i64 = 2_592_000; // 30 days in seconds
    const SPOT_PRICE: f64 = 100.0;
    const HISTORICAL_VOLATILITY: f64 = 0.35;
    const RISK_FREE_RATE: f64 = 0.08;

    fn create_test_params(
        option_type: OptionType,
        strike: f64,
        expiry: i64,
        spot: f64,
        volatility: f64,
        rate: f64,
    ) -> (OptionParams, TokenParams, MarketParams) {
        let option_params = OptionParams {
            option_type,
            strike_price: strike,
            time_until_expiry: expiry,
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
            spot_price: spot,
            historical_volatility: volatility,
            risk_free_rate: rate,
            timestamp: 0,
        };

        let market_params = MarketParams {
            usdc_risk_free_rate: rate,
            time_in_years: expiry as f64 / 31_536_000.0, // Convert i64 seconds to years
            current_timestamp: 0,
        };

        (option_params, token_params, market_params)
    }

    #[test]
    fn test_calc_option_price() {
        let (option_params, token_params, market_params) = create_test_params(
            OPTION_TYPE,
            STRIKE_PRICE,
            TIME_UNTIL_EXPIRY,
            SPOT_PRICE,
            HISTORICAL_VOLATILITY,
            RISK_FREE_RATE,
        );

        let price = calc_option_price(&option_params, &token_params, &market_params);
        println!("Calculated Option Price: {}", price);
        assert!(price > 0.0, "Option price should be greater than 0.0");
    }

    #[test]
    fn test_calc_greeks() {
        let (option_params, token_params, market_params) = create_test_params(
            OPTION_TYPE,
            STRIKE_PRICE,
            TIME_UNTIL_EXPIRY,
            SPOT_PRICE,
            HISTORICAL_VOLATILITY,
            RISK_FREE_RATE,
        );

        let greeks = calc_greeks(&option_params, &token_params, &market_params);
        println!("Calculated Greeks: {:?}", greeks);
        assert!(greeks.delta >= 0.0, "Delta should be >= 0.0 for a LongCall");
    }

    // TO RUN THE TEST: 
    // cargo test -p common --test black_scholes_test -- --nocapture
    
}
