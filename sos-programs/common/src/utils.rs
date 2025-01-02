use anchor_lang::prelude::*;

use common::types::{OptionDataTypes, MarketDataTypes};


pub mod black_scholes_model {
    pub mod standard_normal_variables {
        // CALC

    pub mod greeks {
        use super::*;
        // Calculate Delta 
        pub fn calc_delta(
            token_price: MarketDataTypes::TokenPrice,
            token_volatility: MarketDataTypes::TokenVolatility,
            time_in_years: MarketDataTypes::TimeInYears,
            risk_free_rate: MarketDataTypes::RiskFreeRate,
            strike_price: OptionDataTypes::StrikePrice,
        ) -> OptionDataTypes::OptionGreeks::delta {
            // TODO
        }

    }
}