use anchor_lang::prelude::*;

#[error_code]
pub enum OptionError {
    #[msg("Expiry timestamp must be in the future")]
    InvalidExpiryTimestamp,
    
    #[msg("Strike price must be greater than minimum allowed")]
    InvalidStrikePrice,
    
    #[msg("Volatility must be between MIN_VOLATILITY and MAX_VOLATILITY")]
    InvalidVolatility,
    
    #[msg("Spot price must be greater than zero")]
    InvalidSpotPrice,
    
    #[msg("Risk-free rate must be non-negative")]
    InvalidRiskFreeRate,
    
    #[msg("Option duration must be between 1 day and 1 year")]
    InvalidDuration,
}
