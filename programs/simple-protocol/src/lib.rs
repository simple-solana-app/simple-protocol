use anchor_lang::prelude::*;

declare_id!("DyFiYpt5vU7zeFS3UDs2gq7HuFNBarwMi8PwjKFfbumS");

#[program]
pub mod simple_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
