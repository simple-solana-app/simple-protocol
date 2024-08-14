use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo, borsh1::try_from_slice_unchecked, msg,
    native_token::LAMPORTS_PER_SOL, program_error::ProgramError, pubkey::Pubkey,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{
    solana_program::program_pack::Pack,
    state::{Account as TokenAccount, Mint},
};

use crate::{
    accounts_init::{
        percent_tracker::initialize_percent_tracker_account,
        tracker::Tracker,
        transfer_signer::initialize_transfer_signer_account,
        user_claim_tracker::initialize_user_claim_tracker_account,
        wsol_amount::{initialize_wsol_amount_account, WsolAmount},
    },
    common::{
        PROGRAM_SIMPLE_ASS_TOKEN_ACCOUNT_INITIAL_AMOUNT, RAYDIUM_LP_MINT,
        RAYDIUM_POOL_WSOL_TOKEN_ACCOUNT_PUBKEY, SIMPLE_MINT, SIMPLE_PUBKEY,
    },
    simple_errors::SimpleProtocolErrors,
};

#[allow(clippy::too_many_arguments)]
pub fn execute<'a>(
    program_id: &Pubkey,
    simple: &'a AccountInfo<'a>,
    user: &'a AccountInfo<'a>,
    percent_tracker_pda: &'a AccountInfo<'a>,
    wsol_amount_pda: &'a AccountInfo<'a>,
    transfer_signer_pda: &'a AccountInfo<'a>,
    user_claim_tracker_pda: &'a AccountInfo<'a>,
    program_simple_token_ass_account: &'a AccountInfo<'a>,
    user_simple_token_ass_account: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,
    simple_token_mint_account: &'a AccountInfo<'a>,
    raydium_pool_wsol_token_account: &'a AccountInfo<'a>,
    raydium_pool_lp_token_mint_account: &'a AccountInfo<'a>,
    user_raydium_pool_lp_ass_token_account: &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {
    if simple.is_signer
        && **percent_tracker_pda.lamports.borrow() == 0
        && simple.key.to_string().as_str() == SIMPLE_PUBKEY
    {
        if let Err(e) = initialize_percent_tracker_account(
            program_id,
            simple,
            percent_tracker_pda,
            system_program,
        ) {
            msg!("Failed to initialize percent tracker account: {:?}", e)
        }
    }

    if simple.is_signer
        && **wsol_amount_pda.lamports.borrow() == 0
        && simple.key.to_string().as_str() == SIMPLE_PUBKEY
    {
        if let Err(e) =
            initialize_wsol_amount_account(program_id, simple, wsol_amount_pda, system_program)
        {
            msg!("Failed to initialize WSOL balance account: {:?}", e)
        }
    }

    if simple.is_signer
        && **transfer_signer_pda.lamports.borrow() == 0
        && simple.key.to_string().as_str() == SIMPLE_PUBKEY
    {
        if let Err(e) = initialize_transfer_signer_account(
            program_id,
            simple,
            transfer_signer_pda,
            system_program,
        ) {
            msg!("Failed to initialize Transfer Signer account: {:?}", e)
        }
    }

    if user.is_signer && **user_claim_tracker_pda.lamports.borrow() == 0 {
        if let Err(e) = initialize_user_claim_tracker_account(
            program_id,
            user,
            user_claim_tracker_pda,
            system_program,
        ) {
            msg!("Failed to initialize User Claim account: {:?}", e)
        }
    }

    let user_simple_ata = get_associated_token_address(user.key, simple_token_mint_account.key);
    let user_raydium_pool_lp_ata =
        get_associated_token_address(user.key, raydium_pool_lp_token_mint_account.key);

    if user.is_signer
        && **user_simple_token_ass_account.lamports.borrow() != 0
        && user_simple_token_ass_account.key == &user_simple_ata
        && raydium_pool_wsol_token_account.key.to_string().as_str()
            == RAYDIUM_POOL_WSOL_TOKEN_ACCOUNT_PUBKEY
        && user_raydium_pool_lp_ass_token_account.key == &user_raydium_pool_lp_ata
        && simple_token_mint_account.key.to_string().as_str() == SIMPLE_MINT
        && raydium_pool_lp_token_mint_account.key.to_string().as_str() == RAYDIUM_LP_MINT
    {
        let program_simple_token_ass_account_amount =
            TokenAccount::unpack(&program_simple_token_ass_account.data.borrow())
                .unwrap()
                .amount;

        let mut percent_tracker_account_data =
            try_from_slice_unchecked::<Tracker>(&percent_tracker_pda.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?;

        let total_drainable_simple = program_simple_token_ass_account_amount as f64
            * percent_tracker_account_data.increment as f64
            / 100.0;

        if (program_simple_token_ass_account_amount as f64)
            < (PROGRAM_SIMPLE_ASS_TOKEN_ACCOUNT_INITIAL_AMOUNT - total_drainable_simple)
        {
            msg!("Max drainable simple has already been claimed for this round");
            return Err(SimpleProtocolErrors::MaxSimpleDrained.into());
        }

        let raydium_pool_wsol_token_account_amount =
            TokenAccount::unpack(&raydium_pool_wsol_token_account.data.borrow())
                .unwrap()
                .amount;

        let total_wsol_in_pools = raydium_pool_wsol_token_account_amount; // future simple native pool impl?

        let mut wsol_amount_account_data =
            try_from_slice_unchecked::<WsolAmount>(&wsol_amount_pda.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?;

        if total_wsol_in_pools >= wsol_amount_account_data.amount + 50_000_000_000 {
            wsol_amount_account_data.amount = total_wsol_in_pools;
            wsol_amount_account_data
                .serialize(&mut &mut wsol_amount_pda.data.borrow_mut()[..])
                .map_err(|_| ProgramError::InvalidAccountData)?;

            percent_tracker_account_data.increment += 1;
            percent_tracker_account_data
                .serialize(&mut &mut percent_tracker_pda.data.borrow_mut()[..])
                .map_err(|_| ProgramError::InvalidAccountData)?;
        }

        let mut user_claim_tracker_account_data =
            try_from_slice_unchecked::<Tracker>(&user_claim_tracker_pda.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?;

        if user_claim_tracker_account_data.increment < percent_tracker_account_data.increment {
            let user_raydium_pool_lp_ass_token_account_amount =
                TokenAccount::unpack(&user_raydium_pool_lp_ass_token_account.data.borrow())
                    .unwrap()
                    .amount;

            let raydium_pool_lp_token_mint_supply =
                Mint::unpack(&raydium_pool_lp_token_mint_account.data.borrow())
                    .map_err(|_| ProgramError::InvalidAccountData)?
                    .supply;

            let user_raydium_lp_ratio = user_raydium_pool_lp_ass_token_account_amount as f64
                / raydium_pool_lp_token_mint_supply as f64;

            let user_claim_percent =
                percent_tracker_account_data.increment - user_claim_tracker_account_data.increment;

            let user_share =
                total_drainable_simple * user_claim_percent as f64 / 100.0 * user_raydium_lp_ratio;

            msg!(
                "user claim incr. start: {}",
                user_claim_tracker_account_data.increment
            );
            msg!(
                "percent tracker: {}",
                percent_tracker_account_data.increment
            );
            msg!("user's claim percent: {}%", user_claim_percent);
            msg!(
                "max amount of simple claimable by user: {} simple",
                user_claim_percent as f64 / 100.0 * total_drainable_simple
                    / LAMPORTS_PER_SOL as f64
            );
            msg!("user's LP ratio: {}", user_raydium_lp_ratio);
            msg!(
                "user's actual share: {} simple",
                user_share / LAMPORTS_PER_SOL as f64
            );

            user_claim_tracker_account_data.increment = percent_tracker_account_data.increment;
            user_claim_tracker_account_data
                .serialize(&mut &mut user_claim_tracker_pda.data.borrow_mut()[..])
                .map_err(|_| ProgramError::InvalidAccountData)?;

            msg!(
                "user claim incr. end {}",
                user_claim_tracker_account_data.increment
            )

            //transfer
        }

        msg!(
            "WSOL Amount Tracker: {} ({}) amount: {}",
            wsol_amount_pda.key,
            wsol_amount_pda.owner,
            wsol_amount_account_data.amount / LAMPORTS_PER_SOL
        );

        msg!(
            "Percent Tracker: {} ({}) increment: {}",
            percent_tracker_pda.key,
            percent_tracker_pda.owner,
            percent_tracker_account_data.increment
        );
        msg!(
            "Program's simple token account: {} ({}) amount: {}",
            program_simple_token_ass_account.key,
            program_simple_token_ass_account.owner,
            program_simple_token_ass_account_amount / LAMPORTS_PER_SOL
        );
        msg!(
            "User's Claim Tracker {} ({}) increment: {}",
            user_claim_tracker_pda.key,
            user_claim_tracker_pda.owner,
            user_claim_tracker_account_data.increment,
        );
    }

    Ok(())
}
