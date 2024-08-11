use solana_program::{
    account_info::AccountInfo, borsh1::try_from_slice_unchecked, msg, program_error::ProgramError,
    program_pack::Pack, pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

use crate::accounts_init::{
    percent_tracker::initialize_percent_tracker_account,
    transfer_signer::initialize_transfer_signer_account,
    user_claim_tracker::initialize_user_claim_tracker_account,
    wsol_amount::{initialize_wsol_amount_account, WsolAmount},
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
    system_program: &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {
    if simple.is_signer && **percent_tracker_pda.lamports.borrow() == 0 {
        if let Err(e) = initialize_percent_tracker_account(
            program_id,
            simple,
            percent_tracker_pda,
            system_program,
        ) {
            msg!("Failed to initialize percent tracker account: {:?}", e)
        }
    }

    if simple.is_signer && **wsol_amount_pda.lamports.borrow() == 0 {
        if let Err(e) =
            initialize_wsol_amount_account(program_id, simple, wsol_amount_pda, system_program)
        {
            msg!("Failed to initialize WSOL balance account: {:?}", e)
        }
    }

    if simple.is_signer && **transfer_signer_pda.lamports.borrow() == 0 {
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

    let wsol_balance_account_data =
        try_from_slice_unchecked::<WsolAmount>(&wsol_amount_pda.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;

    let program_simple_token_ass_account_data =
        TokenAccount::unpack(&program_simple_token_ass_account.data.borrow())?;

    msg!(
        "Percent Tracker: {} ({}): {:?}",
        percent_tracker_pda.key,
        percent_tracker_pda.owner,
        percent_tracker_pda.data
    );
    msg!(
        "WSOL Balance: {} ({}): {:?}",
        wsol_amount_pda.key,
        wsol_amount_pda.owner,
        wsol_balance_account_data.amount
    );
    msg!(
        "Program's simple ass token acc: {} ({}): {:?}",
        program_simple_token_ass_account.key,
        program_simple_token_ass_account.owner,
        program_simple_token_ass_account_data.amount,
    );
    msg!(
        "User's claim tracker acc: {} ({}): {:?}",
        user_claim_tracker_pda.key,
        user_claim_tracker_pda.owner,
        user_claim_tracker_pda.data,
    );

    Ok(())
}
