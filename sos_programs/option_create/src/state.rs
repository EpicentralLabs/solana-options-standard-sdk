use anchor_lang::prelude::*;
use common::types::option_data_types::{OptionGreeks, OptionStatus, OptionType};

#[account]
#[derive(Default)]
pub struct OptionAccount {
    pub owner: Pubkey,
    pub option_type: OptionType,
    pub strike_price: f64,
    pub expiry_timestamp: i64,
    pub creation_price: f64,
    pub greeks: OptionGreeks,
    pub status: OptionStatus,
    pub spot_price_at_creation: f64,
}

impl OptionAccount {
    pub const SIZE: usize = 32 + // owner
        8 +  // option_type
        8 +  // strike_price
        8 +  // expiry_timestamp
        8 +  // creation_price
        40 + // greeks (5 * 8)
        1 +  // status
        8;   // spot_price_at_creation
}
