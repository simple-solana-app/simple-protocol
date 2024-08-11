use solana_program::{account_info::AccountInfo, borsh1::try_from_slice_unchecked, msg, program_error::ProgramError};

use crate::accounts_init::program::WsolBalance;

pub fn execute(percent_tracker_pda: &AccountInfo, wsol_balance_pda: &AccountInfo) -> Result<(), ProgramError> {

    let account_data = try_from_slice_unchecked::<WsolBalance>(&wsol_balance_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;

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
        account_data.balance
    );

    Ok(())
}
