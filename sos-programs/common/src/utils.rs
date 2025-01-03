use anchor_lang::prelude::*;
use statrs::distribution::{ContinuousCDF, Normal};

use common::types::{OptionDataTypes, MarketDataTypes};

pub mod black_scholes_model {
    use crate::types::{OptionDataTypes, TokenDataTypes, MarketDataTypes};

    /// Calculates the option price using Black-Scholes model
    pub fn calculate_option_price(
        option_params: &OptionDataTypes::OptionParams,
        token_params: &TokenDataTypes::TokenParams,
        market_params: &MarketDataTypes::MarketParams,
    ) -> f64 {
        let d1 = calculate_d1(
            token_params.spot_price,
            option_params.strike_price,
            token_params.risk_free_rate,
            token_params.historical_volatility,
            calculate_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp),
        );

        let d2 = calculate_d2(
            d1,
            token_params.historical_volatility,
            calculate_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp),
        );

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);

        match option_params.option_type {
            OptionDataTypes::OptionType::LongCall => {
                nd1 * token_params.spot_price - nd2 * option_params.strike_price * (-token_params.risk_free_rate * calculate_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp)).exp()
            }
            OptionDataTypes::OptionType::LongPut => {
                nd2 * option_params.strike_price * (-token_params.risk_free_rate * calculate_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp)).exp() - nd1 * token_params.spot_price
            }
            _ => 0.0, // Handle other option types if necessary
        }
    }

    /// Calculates time to expiry in years
    pub fn calculate_time_to_expiry(
        initial_expiry: i64,
        current_timestamp: i64,
    ) -> f64 {
        // TODO: Convert time difference to years
    }

    /// Calculates option greeks
    pub fn calculate_greeks(
        option_params: &OptionDataTypes::OptionParams,
        token_params: &TokenDataTypes::TokenParams,
        market_params: &MarketDataTypes::MarketParams,
    ) -> OptionDataTypes::OptionGreeks {
        // TODO: Calculate delta, gamma, theta, vega, rho
    }

    /// Checks if option is expired
    pub fn is_option_expired(
        initial_expiry: i64,
        current_timestamp: i64,
    ) -> bool {
        // TODO: Check expiration
    }

    /// Helper function for cumulative normal distribution
    fn normal_cdf(x: f64) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        normal.cdf(x)
    }

    /// Calculate d1 parameter used in Black-Scholes formula:
    /// d₁ = [ln(S/K) + (r + σ²/2)τ] / (σ√τ)
    fn calculate_d1(
        spot_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        volatility: f64,
        time_to_expiry: f64,
    ) -> f64 {
        let numerator = (spot_price / strike_price).ln() + (risk_free_rate + 0.5 * volatility * volatility) * time_to_expiry;
        let denominator = volatility * time_to_expiry.sqrt();
        numerator / denominator
    }

    /// Calculate d2 parameter for Black-Scholes
    /// d2 = d1 - σ√τ
    fn calculate_d2(
        d1: f64,
        volatility: f64,
        time_to_expiry: f64,
    ) -> f64 {
        d1 - volatility * time_to_expiry.sqrt()
    }
}