use anchor_lang::prelude::*;

declare_id!("85aDWaYQbL9X67aHgpJbkTzYshzKe5VjnyjMuJ351Fc9");

#[program]
pub mod sos_programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
