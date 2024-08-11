use solana_program::{
    account_info::AccountInfo, borsh1::try_from_slice_unchecked, msg, program_error::ProgramError
};
use spl_token::{solana_program::program_pack::Pack, state::Account as TokenAccount};


use crate::accounts_init::program::WsolBalance;

pub fn execute(
    percent_tracker_pda: &AccountInfo,
    wsol_balance_pda: &AccountInfo,
    program_simple_token_ass_account: &AccountInfo,
) -> Result<(), ProgramError> {
    let wsol_balance_account_data = try_from_slice_unchecked::<WsolBalance>(&wsol_balance_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let program_simple_token_ass_account_data = TokenAccount::unpack(&program_simple_token_ass_account.data.borrow())?;
    
    msg!(
        "Percent Tracker: {} ({}): {:?}",
        percent_tracker_pda.key,
        percent_tracker_pda.owner,
        percent_tracker_pda.data
    );
    msg!(
        "WSOL Balance: {} ({}): {:?}",
        wsol_balance_pda.key,
        wsol_balance_pda.owner,
        wsol_balance_account_data.balance
    );
    msg!(
        "Program's simple ass token acc: {} ({}): {:?}",
        program_simple_token_ass_account.key,
        program_simple_token_ass_account.owner,
        program_simple_token_ass_account_data.amount,
    );

    Ok(())
}
