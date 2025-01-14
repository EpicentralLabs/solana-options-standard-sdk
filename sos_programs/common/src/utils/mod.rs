// common/src/utils.rs

pub mod black_scholes_model {
    // Import traits & types needed
    use statrs::distribution::{Continuous, ContinuousCDF, Normal};

    // Pull in our own crate's types by fully qualifying them:
    use crate::types::{
        market_data_types::MarketParams,
        option_data_types::{OptionGreeks, OptionParams, OptionType},
        token_data_types::TokenParams,
    };

    /// Example Black–Scholes price function
    pub fn calc_option_price(
        option_params: &OptionParams,
        token_params: &TokenParams,
        market_params: &MarketParams,
    ) -> f64 {
        let time_to_expiry = calc_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp);
        let d1 = calc_d1(
            token_params.spot_price,
            option_params.strike_price,
            token_params.risk_free_rate,
            token_params.historical_volatility,
            time_to_expiry,
        );
        let d2 = d1 - (token_params.historical_volatility * time_to_expiry.sqrt());

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);

        match option_params.option_type {
            // Call price
            OptionType::LongCall => {
                nd1 * token_params.spot_price
                    - nd2 * option_params.strike_price * (-token_params.risk_free_rate * time_to_expiry).exp()
            }
            // Put price (simplistic)
            OptionType::LongPut => {
                let n_negd1 = normal.cdf(-d1);
                let n_negd2 = normal.cdf(-d2);
                option_params.strike_price * (-token_params.risk_free_rate * time_to_expiry).exp() * n_negd2
                    - token_params.spot_price * n_negd1
            }
            // ShortCall, ShortPut, etc. left as 0.0 for simplicity
            _ => 0.0,
        }
    }

    /// Example Greeks calculation
    pub fn calc_greeks(
        option_params: &OptionParams,
        token_params: &TokenParams,
        market_params: &MarketParams,
    ) -> OptionGreeks {
        let time_to_expiry = calc_time_to_expiry(option_params.initial_time_to_expiry, market_params.current_timestamp);
        let d1 = calc_d1(
            token_params.spot_price,
            option_params.strike_price,
            token_params.risk_free_rate,
            token_params.historical_volatility,
            time_to_expiry,
        );
        let d2 = d1 - (token_params.historical_volatility * time_to_expiry.sqrt());

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);
        let npd1 = normal.pdf(d1); // use of Continuous trait

        let delta = match option_params.option_type {
            OptionType::LongCall => nd1,
            OptionType::LongPut => nd1 - 1.0,
            _ => 0.0,
        };

        let gamma = npd1 / (token_params.spot_price * token_params.historical_volatility * time_to_expiry.sqrt());

        let theta = match option_params.option_type {
            OptionType::LongCall => {
                -(token_params.spot_price * npd1 * token_params.historical_volatility) / (2.0 * time_to_expiry.sqrt())
                    - token_params.risk_free_rate * option_params.strike_price
                    * (-token_params.risk_free_rate * time_to_expiry).exp()
                    * nd2
            }
            OptionType::LongPut => {
                -(token_params.spot_price * npd1 * token_params.historical_volatility) / (2.0 * time_to_expiry.sqrt())
                    + token_params.risk_free_rate * option_params.strike_price
                    * (-token_params.risk_free_rate * time_to_expiry).exp()
                    * (1.0 - nd2)
            }
            _ => 0.0,
        };

        let vega = token_params.spot_price * npd1 * time_to_expiry.sqrt();

        let rho = match option_params.option_type {
            OptionType::LongCall => option_params.strike_price * time_to_expiry.exp() * nd2,
            OptionType::LongPut => -option_params.strike_price * time_to_expiry.exp() * (1.0 - nd2),
            _ => 0.0,
        };

        OptionGreeks {
            delta,
            gamma,
            theta,
            vega,
            rho,
        }
    }

    fn calc_time_to_expiry(initial_expiry: i64, current_timestamp: i64) -> f64 {
        let seconds_in_a_year = 31_536_000.0;
        let time_diff = initial_expiry - current_timestamp;
        time_diff as f64 / seconds_in_a_year
    }

    fn calc_d1(
        spot_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        volatility: f64,
        time_to_expiry: f64,
    ) -> f64 {
        let numerator = (spot_price / strike_price).ln()
            + (risk_free_rate + 0.5 * volatility * volatility) * time_to_expiry;
        let denominator = volatility * time_to_expiry.sqrt();
        numerator / denominator
    }
}
