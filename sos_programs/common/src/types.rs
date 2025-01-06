use anchor_lang::prelude::Pubkey;

/// Module for option-related data
pub mod option_data_types {
    use super::Pubkey;

    pub struct OptionAccount {
        pub option_id: Pubkey,
        pub option_status: OptionStatus,
        pub option_params: OptionParams,
    }

    pub enum OptionStatus {
        Open,
        Expired,
        Exercised,
    }

    pub struct OptionParams {
        pub option_type: OptionType,
        pub strike_price: f64,
        pub initial_time_to_expiry: i64,
        pub creation_price: f64,
        pub greeks: OptionGreeks,
    }

    pub enum OptionType {
        LongCall,
        ShortCall,
        LongPut,
        ShortPut,
    }
    #[derive(Debug)]
    pub struct OptionGreeks {
        pub delta: f64,
        pub theta: f64,
        pub gamma: f64,
        pub vega: f64,
        pub rho: f64,
    }
}

/// Module for token-related data
pub mod token_data_types {
    pub struct TokenParams {
        pub spot_price: f64,
        pub historical_volatility: f64,
        pub risk_free_rate: f64,
        pub timestamp: i64,
    }
}

/// Module for market-wide parameters
pub mod market_data_types {
    pub struct MarketParams {
        pub usdc_risk_free_rate: f64,
        pub time_in_years: f64,
        pub current_timestamp: i64,
    }
}
