use anchor_lang::prelude::*;

declare_id!("7nFk9yFhpEKiUarQhqtmGPJ2MDmhoxX5rfLmehVAyPi2");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
