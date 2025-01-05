use anchor_lang::prelude::*;

pub mod OptionDataTypes {
    pub struct OptionAccount { // Main account storing option contract data
        pub option_id: Pubkey,
        pub option_status: OptionStatus,
        pub option_params: OptionParams,
    }

    pub enum OptionStatus { // The status of an option
        Open, // The option is still active
        Expired, // The option has expired
        Exercised, // The option has been exercised
    }

    pub struct OptionParams { // An Account that stores the parameters of an option contract
        pub option_type: OptionType, // The type of option
        pub strike_price: f64, // The strike price of the option
        pub initial_time_to_expiry: i64, // How much time is left until the option expires (set with blockchain timestamp)
        pub creation_price: f64, // Add initial price when option created
        pub greeks: OptionGreeks, // The greeks of the option
    }

    pub enum OptionType { // The type of option
        LongCall, // Buying a Call Option
        ShortCall, // Selling a Call Option 
        LongPut, // Buying a Put Option
        ShortPut, // Selling a Put Option
    }

    pub struct OptionGreeks { // An account that stores the data for the greeks of an option
        pub delta: f64, // Rate of change between option price and underlying asset price
        pub theta: f64, // Rate of change in option value with respect to time
        pub gamma: f64, // Rate of change in delta with respect to underlying price
        pub vega: f64, // Rate of change in option value with respect to volatility
        pub rho: f64, // Rate of change in option value with respect to interest rate
    }
}

pub mod TokenDataTypes { // Module containing token-related data structures
    pub struct TokenParams { // Parameters related to the underlying token
        pub spot_price: f64, // Current market price of the token
        pub historical_volatility: f64, // Historical price volatility of the token
        pub risk_free_rate: f64, // Risk-free interest rate for the token
        pub timestamp: i64, // Current timestamp for the token data (set with blockchain timestamp)
    }
}

pub mod MarketDataTypes { // Module containing market-wide parameters
    pub struct MarketParams { // Global market parameters
        pub usdc_risk_free_rate: f64, // Risk-free rate in USDC terms
        pub time_in_years: f64, // Time expressed in years
        pub current_timestamp: i64, // Current market timestamp
    }
}