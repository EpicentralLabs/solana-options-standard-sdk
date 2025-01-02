use anchor_lang::prelude::*;

pub mod OptionDataTypes {
    pub struct OptionAccount {
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
        pub time_to_expiry: u64, // How much time is left until the option expires
        pub greeks: OptionGreeks, // The greeks of the option

    }

    pub enum OptionType { // The type of option
        LongCall, // Buying a Call Option
        ShortCall, // Selling a Call Option 
        LongPut, // Buying a Put Option
        ShortPut, // Selling a Put Option
    }
    pub struct OptionGreeks { // An account that stores the data for the greeks of an option
        pub delta: f64,
        pub theta: f64,
        pub gamma: f64,
        pub vega: f64,
        pub rho: f64,
    }
}

pub mod MarketDataTypes { // Module containing market-related data structures
    pub struct TokenPrice { // Current price of the underlying token
        pub token_price: f64,
    }

    pub struct TokenVolatility { // Volatility measure of the token price
        pub token_volatility: f64,
    }

    pub struct TimeInYears { // Time duration in years
        pub time_in_years: u64,
    }

    pub struct RiskFreeRate { // Annual risk-free interest rate of the underlying token
        pub risk_free_rate: f64,
    }
}