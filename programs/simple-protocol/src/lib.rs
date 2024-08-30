use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("DyFiYpt5vU7zeFS3UDs2gq7HuFNBarwMi8PwjKFfbumS");

pub const SIMPLE_PUBKEY: &str = "E61fUAd1cxFES9kPckPhzwiiFMRo8ezAw7ZG5a8YD2jv";
pub const SIMPLE_MINT: &str = "9CigozmpiDkUCXBjWojV1hi4jj6Sc47LXfs3aXKjhv2j";


#[program]
pub mod simple_protocol {
    use super::*;

    pub fn initialize(ctx: Context<InitializeProgramAccounts>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
struct InitializeProgramAccounts<'info> {
    #[account(
        mut,
        address = Pubkey::from_str(SIMPLE_PUBKEY).unwrap()
    )]
    simple: Signer<'info>,
    #[account(
        init,
        payer = simple,
        space = 8 + 1,
        seeds = [b"percent_tracker"],
        bump
    )]
    percent_tracker: Account<'info, Tracker>,
    #[account(
        init,
        payer = simple,
        space = 8 + 8,
        seeds = [b"wsol_balance"],
        bump
    )]
    wsol_balance: Account<'info, Balance>,
    #[account(
        init,
        payer = simple,
        space = 8,
        seeds = [b"transfer_authority"],
        bump
    )]
    transfer_authority: Account<'info, Authority>,
    #[account(address = Pubkey::from_str(SIMPLE_MINT).unwrap())]
    simple_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = simple,
        token::mint = simple_mint,
        token::authority = transfer_authority,
    )]
    program_simple_token: Account<'info, TokenAccount>,
    system_program: Program<'info, System>
    
}

#[account]
pub struct Tracker {
    increment: u8,
}

#[account]
pub struct Balance {
    amount: u64,
}

#[account]
pub struct Authority {}