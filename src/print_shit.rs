use solana_program::{account_info::{self, next_account_info, AccountInfo}, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult, msg, program_error::ProgramError};

use crate::accounts_init::{program::wsol_amount::WsolAmount, Tracker};

pub fn print_shit(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let simple = next_account_info(account_info_iter)?;
    let percent_tracker_pda = next_account_info(account_info_iter)?;
    let wsol_amount_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let percent_tracker_pda_increment = try_from_slice_unchecked::<Tracker>(&percent_tracker_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?.increment;
    
    let wsol_amount_pda_amount = try_from_slice_unchecked::<WsolAmount>(&wsol_amount_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?.amount;

    msg!("simple: {}, ({})", simple.key, simple.owner);
    msg!("percent_tracker_pda: {}, ({}): {}", percent_tracker_pda.key, percent_tracker_pda.owner, percent_tracker_pda_increment);
    msg!("wsol_amount_pda: {}, ({}): {}", wsol_amount_pda.key, wsol_amount_pda.owner, wsol_amount_pda_amount);
    msg!("system_program: {}, ({})", system_program.key, system_program.owner);
    Ok(())
}
