use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("BbYJTNjp7BHMdBcy83kVWCbagB2iN26ynKphPK5DZFxG");

pub const SIMPLE_PUBKEY: &str = "E61fUAd1cxFES9kPckPhzwiiFMRo8ezAw7ZG5a8YD2jv";
pub const SIMPLE_MINT: &str = "BKPHSeJ4DmQnnT7NwoqirjJaM6GVyxJJyYoHd7TA4hsD";
pub const RAYDIUM_POOL_WSOL_TOKEN_ACCOUNT: &str = "6XY1UXw8i4ZydPL4B4Wid2kNDJ7XTDYmFx7Q9ULCUYFJ";
pub const RAYDIUM_LP_MINT: &str = "B9F82waRKg622E9FJN9fLN43TNDctGEAqKvCpWvccgvB";
pub const CREATOR_SIMPLE_TOKEN_ACCOUNT: &str = "8MQDmMgGFV3cA2LBtep1zwh5Sftyk6pw7yzgRjHc4okM";
pub const PROGRAM_SIMPLE_TOKEN_ACCOUNT_INITIAL_AMOUNT: u64 = 415420420696969666;

#[error_code]
pub enum SimpleProtocolError {
    #[msg("Uninitialized account")]
    UninitializedAccount,
    #[msg("Zero LP tokens")]
    ZeroLpTokens,
    #[msg("Max simple drained for now")]
    MaxSimpleDrainedForNow,
}

#[program]
pub mod simple_protocol {
    use {
        super::*,
        anchor_lang::{prelude::Context, solana_program::native_token::LAMPORTS_PER_SOL},
        anchor_spl::token::{transfer, Transfer},
    };

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
        _ctx: Context<InitializeRemainingProgramAccounts>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn initalize_user_claim_tracker(ctx: Context<InitializeUserClaimTracker>) -> Result<()> {
        let user_claim_tracker = &mut ctx.accounts.user_claim_tracker;
        user_claim_tracker.increment = 0;

        Ok(())
    }

    pub fn execute(ctx: Context<Execute>) -> Result<()> {
        let user = &mut ctx.accounts.user;

        msg!("user {} ({})", user.key(), user.to_account_info().owner,);

        let percent_tracker = &mut ctx.accounts.percent_tracker;
        let wsol_balance = &mut ctx.accounts.wsol_balance;

        let transfer_authority = &ctx.accounts.transfer_authority;
        let bump_seed = ctx.bumps.transfer_authority;
        let signer_seeds: &[&[&[u8]]] = &[&[b"transfer_authority", &[bump_seed]]];
        let program_simple_token_account = &mut ctx.accounts.program_simple_token_account;

        let user_claim_tracker = &mut ctx.accounts.user_claim_tracker;
        let user_simple_token_account = &mut ctx.accounts.user_simple_token_account;
        let user_raydium_lp_ata = &ctx.accounts.user_raydium_lp_ata;

        let raydium_pool_wsol_token_account = &ctx.accounts.raydium_pool_wsol_token_account;
        let raydium_lp_mint = &ctx.accounts.raydium_lp_mint;

        let creator_simple_token_account = &mut ctx.accounts.creator_simple_token_account;

        let token_program = &ctx.accounts.token_program;

        let total_drainable_simple =
            program_simple_token_account.amount as f64 * percent_tracker.increment as f64 / 100.0;

        msg!(
            "percent_tracker {} ({}): {}%",
            percent_tracker.key(),
            percent_tracker.to_account_info().owner,
            percent_tracker.increment
        );

        msg!(
            "wsol_balance {} ({}): {} WSOL",
            wsol_balance.key(),
            wsol_balance.to_account_info().owner,
            wsol_balance.amount / LAMPORTS_PER_SOL
        );

        msg!(
            "raydium_pool_wsol_token_account {} ({}): {} WSOL",
            raydium_pool_wsol_token_account.key(),
            raydium_pool_wsol_token_account.to_account_info().owner,
            raydium_pool_wsol_token_account.amount / LAMPORTS_PER_SOL
        );

        msg!(
            "transfer_authority {} ({})",
            transfer_authority.key(),
            transfer_authority.to_account_info().owner
        );

        msg!(
            "program_simple_account {} ({}): {} simple",
            program_simple_token_account.key(),
            program_simple_token_account.to_account_info().owner,
            program_simple_token_account.amount / LAMPORTS_PER_SOL
        );

        msg!(
            "total_drainable_simple {} simple",
            total_drainable_simple / LAMPORTS_PER_SOL as f64
        );

        msg!(
            "There can't be less than {} - {} = {} simple in pool",
            PROGRAM_SIMPLE_TOKEN_ACCOUNT_INITIAL_AMOUNT as f64 / LAMPORTS_PER_SOL as f64,
            total_drainable_simple / LAMPORTS_PER_SOL as f64,
            PROGRAM_SIMPLE_TOKEN_ACCOUNT_INITIAL_AMOUNT as f64 / LAMPORTS_PER_SOL as f64
                - total_drainable_simple / LAMPORTS_PER_SOL as f64
        );

        msg!(
            "user_claim_tracker {} ({}): {}",
            user_claim_tracker.key(),
            user_claim_tracker.to_account_info().owner,
            user_claim_tracker.increment
        );

        msg!(
            "user_simple_ata {} ({}): {} simple",
            user_simple_token_account.key(),
            user_simple_token_account.to_account_info().owner,
            user_simple_token_account.amount / LAMPORTS_PER_SOL
        );
        if **user_claim_tracker.to_account_info().lamports.borrow() == 0
            || **user_simple_token_account
                .to_account_info()
                .lamports
                .borrow()
                == 0
            || **user_raydium_lp_ata.to_account_info().lamports.borrow() == 0
        {
            return Err(SimpleProtocolError::UninitializedAccount.into());
        } else if user_raydium_lp_ata.amount == 0 {
            return Err(SimpleProtocolError::ZeroLpTokens.into());

        } else if (program_simple_token_account.amount as f64)
            < (PROGRAM_SIMPLE_TOKEN_ACCOUNT_INITIAL_AMOUNT as f64 - total_drainable_simple)
        {
            return Err(SimpleProtocolError::MaxSimpleDrainedForNow.into());
        } else {
            if raydium_pool_wsol_token_account.amount >= wsol_balance.amount + 50_000_000_000 {
                wsol_balance.amount = raydium_pool_wsol_token_account.amount;
                percent_tracker.increment += 1;
            }

            if user_claim_tracker.increment < percent_tracker.increment {
                let user_claim_percent = percent_tracker.increment - user_claim_tracker.increment;
                let user_lp_ratio =
                    user_raydium_lp_ata.amount as f64 / raydium_lp_mint.supply as f64;
                let user_drainable_simple = program_simple_token_account.amount as f64 * user_claim_percent as f64 / 100.0;

                let user_share =
                    user_drainable_simple * user_lp_ratio;

                let simple_share = user_share / 100.0;

                let real_user_share = user_share - simple_share;

                transfer(
                    CpiContext::new_with_signer(
                        token_program.to_account_info(),
                        Transfer {
                            from: program_simple_token_account.to_account_info(),
                            to: user_simple_token_account.to_account_info(),
                            authority: transfer_authority.to_account_info(),
                        },
                        signer_seeds,
                    ),
                    real_user_share as u64,
                )?;

                transfer(
                    CpiContext::new_with_signer(
                        token_program.to_account_info(),
                        Transfer {
                            from: program_simple_token_account.to_account_info(),
                            to: creator_simple_token_account.to_account_info(),
                            authority: transfer_authority.to_account_info(),
                        },
                        signer_seeds,
                    ),
                    simple_share as u64,
                )?;

                user_claim_tracker.increment = percent_tracker.increment;

                msg!(
                    "user_raydium_lp_ata {} ({}): {} Raydium LP",
                    user_raydium_lp_ata.key(),
                    user_raydium_lp_ata.owner,
                    user_raydium_lp_ata.amount / LAMPORTS_PER_SOL
                );
                msg!(
                    "raydium_lp_mint {} ({}): {} Raydium LP Supply",
                    raydium_lp_mint.key(),
                    raydium_lp_mint.to_account_info().owner,
                    raydium_lp_mint.supply / LAMPORTS_PER_SOL
                );

                msg!(
                    "user_drainable_simple {} simple",
                    user_drainable_simple / LAMPORTS_PER_SOL as f64,
                );
                msg!(
                    "user gets {} of {} simple",
                    user_lp_ratio,
                    user_drainable_simple / LAMPORTS_PER_SOL as f64,
                );

                msg!(
                    "full user_share: {} simple",
                    user_share / LAMPORTS_PER_SOL as f64
                );
                msg!(
                    "real_user_share: {} simple",
                    real_user_share / LAMPORTS_PER_SOL as f64
                );
                msg!(
                    "simple_share: {} simple",
                    simple_share / LAMPORTS_PER_SOL as f64
                )
            }
        }

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
        seeds = [b"transfer_authority"],
        bump
    )]
    transfer_authority: Account<'info, Authority>,
    #[account(
        mut,
        token::mint = simple_mint,
        token::authority = transfer_authority,
        seeds = [b"program_simple_token_account"],
        bump
    )]
    program_simple_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [user.key.as_ref()],
        bump
    )]
    user_claim_tracker: Account<'info, Tracker>,
    #[account(
        mut,
        token::mint = simple_mint,
    )]
    user_simple_token_account: Account<'info, TokenAccount>,
    #[account(token::mint = raydium_lp_mint,)]
    user_raydium_lp_ata: Account<'info, TokenAccount>,
    #[account(address = Pubkey::from_str(RAYDIUM_POOL_WSOL_TOKEN_ACCOUNT).unwrap())]
    raydium_pool_wsol_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        address = Pubkey::from_str(CREATOR_SIMPLE_TOKEN_ACCOUNT).unwrap()
    )]
    creator_simple_token_account: Account<'info, TokenAccount>,
    #[account(address = Pubkey::from_str(SIMPLE_MINT).unwrap())]
    simple_mint: Account<'info, Mint>,
    #[account(address = Pubkey::from_str(RAYDIUM_LP_MINT).unwrap())]
    raydium_lp_mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
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
    #[account(
        init,
        payer = simple,
        token::mint = simple_mint,
        token::authority = transfer_authority,
        seeds = [b"program_simple_token_account"],
        bump
    )]
    program_simple_token_account: Account<'info, TokenAccount>,
    #[account(address = Pubkey::from_str(SIMPLE_MINT).unwrap())]
    simple_mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUserClaimTracker<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 1,
        seeds = [user.key.as_ref()],
        bump
    )]
    user_claim_tracker: Account<'info, Tracker>,
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
