use anchor_lang::prelude::*;

declare_id!("85aDWaYQbL9X67aHgpJbkTzYshzKe5VjnyjMuJ351Fc9");


#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
