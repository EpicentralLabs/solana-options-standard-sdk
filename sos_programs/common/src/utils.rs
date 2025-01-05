use anchor_lang::prelude::*;
use statrs::distribution::{ContinuousCDF, Normal}; // For normal distribution calculations
use common::types::{OptionDataTypes, MarketDataTypes};

pub mod black_scholes_model {
    use crate::types::{OptionDataTypes, TokenDataTypes, MarketDataTypes};

    /// Calculates the option price using Black-Scholes model
    /// Returns the theoretical price of the option based on the input parameters
    pub fn calc_option_price(
        option_params: &OptionDataTypes::OptionParams,
        token_params: &TokenDataTypes::TokenParams,
        market_params: &MarketDataTypes::MarketParams,
    ) -> f64 {
        // Calculate d1 and d2 parameters used in Black-Scholes formula
        let d1 = calc_d1(
            token_params.spot_price,
            option_params.strike_price,
            token_params.risk_free_rate,
            token_params.historical_volatility,
            calc_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp),
        );

        let d2 = calc_d2(
            d1,
            token_params.historical_volatility,
            calc_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp),
        );

        // Calculate cumulative normal distribution values
        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);

        // Calculate option price based on option type
        match option_params.option_type {
            OptionDataTypes::OptionType::LongCall => {
                nd1 * token_params.spot_price - nd2 * option_params.strike_price * (-token_params.risk_free_rate * calc_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp)).exp()
            }
            OptionDataTypes::OptionType::LongPut => {
                nd2 * option_params.strike_price * (-token_params.risk_free_rate * calc_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp)).exp() - nd1 * token_params.spot_price
            }
            _ => 0.0, // Handle other option types if necessary
        }
    }

    /// Calculates time to expiry in years
    /// 
    /// # Parameters
    /// - `initial_expiry`: The initial expiry timestamp in seconds.
    /// - `current_timestamp`: The current timestamp in seconds.
    ///
    /// # Returns
    /// - The time to expiry in years.
    pub fn calc_time_to_expiry(
        initial_expiry: i64,
        current_timestamp: i64,
    ) -> f64 {
        let seconds_in_a_year = 31_536_000.0; // 365 days * 24 hours * 60 minutes * 60 seconds
        let time_difference = initial_expiry - current_timestamp;
        time_difference as f64 / seconds_in_a_year
    }

    /// Calculates option greeks
    /// Calculates the Greeks (delta, gamma, theta, vega, rho) for an option
    /// using the Black-Scholes model parameters
    pub fn calc_greeks(
        option_params: &OptionDataTypes::OptionParams,
        token_params: &TokenDataTypes::TokenParams,
        market_params: &MarketDataTypes::MarketParams,
    ) -> OptionDataTypes::OptionGreeks {
        let time_to_expiry = calc_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp);
        
        // Calculate d1 and d2 parameters
        let d1 = calc_d1(
            token_params.spot_price,
            option_params.strike_price,
            token_params.risk_free_rate,
            token_params.historical_volatility,
            time_to_expiry,
        );

        let d2 = calc_d2(
            d1,
            token_params.historical_volatility,
            time_to_expiry,
        );

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1); // Cumulative distribution function
        let npd1 = normal.pdf(d1); // Probability density function for d1

        // Delta: First derivative of option price with respect to underlying price
        let delta = match option_params.option_type {
            OptionDataTypes::OptionType::LongCall => nd1,
            OptionDataTypes::OptionType::LongPut => nd1 - 1.0,
            _ => 0.0,
        };

        // Gamma: Second derivative of option price with respect to underlying price
        // Measures rate of change in delta
        let gamma = npd1 / (token_params.spot_price * token_params.historical_volatility * time_to_expiry.sqrt());

        // Theta: Rate of change in option value with respect to time
        // Measures time decay of option value
        let theta = match option_params.option_type {
            OptionDataTypes::OptionType::LongCall => {
                -(token_params.spot_price * npd1 * token_params.historical_volatility) / (2.0 * time_to_expiry.sqrt())
                - token_params.risk_free_rate * option_params.strike_price * (-token_params.risk_free_rate * time_to_expiry).exp() * normal.cdf(d2)
            }
            OptionDataTypes::OptionType::LongPut => {
                -(token_params.spot_price * npd1 * token_params.historical_volatility) / (2.0 * time_to_expiry.sqrt())
                + token_params.risk_free_rate * option_params.strike_price * (-token_params.risk_free_rate * time_to_expiry).exp() * (1.0 - normal.cdf(d2))
            }
            _ => 0.0,
        };

        // Vega: Sensitivity of option price to changes in volatility
        // Measures impact of volatility changes
        let vega = token_params.spot_price * npd1 * time_to_expiry.sqrt();

        // Rho: Sensitivity of option price to changes in risk-free rate
        // Measures impact of interest rate changes
        let rho = match option_params.option_type {
            OptionDataTypes::OptionType::LongCall => option_params.strike_price * time_to_expiry.exp() * normal.cdf(d2),
            OptionDataTypes::OptionType::LongPut => -option_params.strike_price * time_to_expiry.exp() * (1.0 - normal.cdf(d2)),
            _ => 0.0,
        };

        OptionDataTypes::OptionGreeks {
            delta,
            gamma,
            theta,
            vega,
            rho,
        }
    }

    /// Checks if option is expired by comparing timestamps
    pub fn is_option_expired(
        initial_expiry: i64,
        current_timestamp: i64,
    ) -> bool {
        current_timestamp >= initial_expiry
    }

    /// Helper function for cumulative normal distribution
    /// Returns probability that a value drawn from standard normal distribution is <= x
    fn normal_cdf(x: f64) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        normal.cdf(x)
    }

    /// Calculate d1 parameter used in Black-Scholes formula:
    /// d₁ = [ln(S/K) + (r + σ²/2)τ] / (σ√τ)
    /// Where: S = spot price, K = strike price, r = risk-free rate,
    /// σ = volatility, τ = time to expiry
    fn calc_d1(
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
    /// Where: d1 = calculated d1 parameter, σ = volatility, τ = time to expiry
    fn calc_d2(
        d1: f64,
        volatility: f64,
        time_to_expiry: f64,
    ) -> f64 {
        d1 - volatility * time_to_expiry.sqrt()
    }
}