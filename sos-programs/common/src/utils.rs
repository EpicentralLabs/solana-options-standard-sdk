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
        // TODO: Implement Black-Scholes formula
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
    /// where:
    /// d₁ represents a standardized measure that helps determine 
    /// the probability of option exercise, adjusted for the time value
    /// of money and volatility risk premium
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
    /// where:
    /// - d1 is the d1 parameter previously calculated
    /// - σ is the volatility of the underlying asset
    /// - τ is the time to expiry in years
    fn calculate_d2(
        d1: f64,
        volatility: f64,
        time_to_expiry: f64,
    ) -> f64 {
        d1 - volatility * time_to_expiry.sqrt()
    }
}