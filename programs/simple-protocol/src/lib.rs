use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("DyFiYpt5vU7zeFS3UDs2gq7HuFNBarwMi8PwjKFfbumS");

pub const SIMPLE_PUBKEY: &str = "E61fUAd1cxFES9kPckPhzwiiFMRo8ezAw7ZG5a8YD2jv";
pub const SIMPLE_MINT: &str = "GL3E99ERZBe68mYXrJZfEoSWwoY4QbCWT5H6Jvb7E5RC";

#[program]
pub mod simple_protocol {
    use anchor_spl::token::{transfer, Transfer};

    use super::*;

    pub fn initialize_most_program_accounts(
        ctx: Context<InitializeMostProgramAccounts>,
    ) -> Result<()> {
        let percent_tracker = &mut ctx.accounts.percent_tracker;
        percent_tracker.increment = 0;

        let wsol_balance = &mut ctx.accounts.wsol_balance;
        wsol_balance.amount = 0;
        Ok(())
    }

    pub fn initialize_remaining_program_accounts(
        ctx: Context<InitializeRemainingProgramAccounts>,
    ) -> Result<()> {
        let transfer_authority = &ctx.accounts.transfer_authority;
        let program_simple_account = &ctx.accounts.program_simple_token;

        msg!(
            "transfer_authority {} ({})",
            transfer_authority.key(),
            transfer_authority.to_account_info().owner
        );
        msg!(
            "program_simple_account {} ({}): {}",
            program_simple_account.key(),
            program_simple_account.to_account_info().owner,
            program_simple_account.amount
        );

        Ok(())
    }

    pub fn execute(ctx: Context<Execute>) -> Result<()> {
        let percent_tracker = &mut ctx.accounts.percent_tracker;
        let wsol_balance = &mut ctx.accounts.wsol_balance;

        let transfer_authority = &ctx.accounts.transfer_authority;
        let bump_seed = ctx.bumps.transfer_authority;
        let signer_seeds: &[&[&[u8]]] = &[&[b"transfer_authority", &[bump_seed]]];
        let program_simple_account = &mut ctx.accounts.program_simple_token;

        let dest = &mut ctx.accounts.dest;

        let token_program = &ctx.accounts.token_program;

        transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: program_simple_account.to_account_info(),
                    to: dest.to_account_info(),
                    authority: transfer_authority.to_account_info(),
                },
                signer_seeds,
            ),
            10,
        )?;

        msg!(
            "percent_tracker {} ({}): {}",
            percent_tracker.key(),
            percent_tracker.to_account_info().owner,
            percent_tracker.increment
        );
        msg!(
            "wsol_balance {} ({}): {}",
            wsol_balance.key(),
            wsol_balance.to_account_info().owner,
            wsol_balance.amount
        );
        msg!(
            "transfer_authority {} ({})",
            transfer_authority.key(),
            transfer_authority.to_account_info().owner
        );
        msg!(
            "program_simple_account {} ({}): {}",
            program_simple_account.key(),
            program_simple_account.to_account_info().owner,
            program_simple_account.amount
        );
        msg!(
            "dest {} ({}): {}",
            dest.key(),
            dest.to_account_info().owner,
            dest.amount
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"percent_tracker"],
        bump
    )]
    percent_tracker: Account<'info, Tracker>,
    #[account(
        mut,
        seeds = [b"wsol_balance"],
        bump
    )]
    wsol_balance: Account<'info, Balance>,
    #[account(
        mut,
        seeds = [b"transfer_authority"],
        bump
    )]
    transfer_authority: Account<'info, Authority>,
    #[account(address = Pubkey::from_str(SIMPLE_MINT).unwrap())]
    simple_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = simple_mint,
        token::authority = transfer_authority,
        seeds = [b"program_simple_token_account"],
        bump
    )]
    program_simple_token: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    #[account(
        mut,
    )]
    dest: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct InitializeMostProgramAccounts<'info> {
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
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeRemainingProgramAccounts<'info> {
    #[account(
        mut,
        address = Pubkey::from_str(SIMPLE_PUBKEY).unwrap()
    )]
    simple: Signer<'info>,
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
        seeds = [b"program_simple_token_account"],
        bump
    )]
    program_simple_token: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[account]
struct Tracker {
    increment: u8,
}

#[account]
struct Balance {
    amount: u64,
}

#[account]
struct Authority {}
