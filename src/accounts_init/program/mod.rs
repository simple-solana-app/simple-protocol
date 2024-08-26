use {
    crate::{common::SIMPLE_PUBKEY, error::SimpleProtocolError},
    percent_tracker::initialize_percent_tracker_account,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        pubkey::Pubkey,
    },
    wsol_amount::initialize_wsol_amount_account,
};

pub mod percent_tracker;
pub mod transfer_signer;
pub mod wsol_amount;

pub fn init_prog_accounts(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let simple = next_account_info(account_info_iter)?;
    let percent_tracker_pda = next_account_info(account_info_iter)?;
    let wsol_amount_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Only proceed if `simple.is_signer` and `simple.key` matches `SIMPLE_PUBKEY`
    if simple.is_signer && simple.key.to_string().as_str() == SIMPLE_PUBKEY {
        
        if **percent_tracker_pda.lamports.borrow() == 0 {
            initialize_percent_tracker_account(
                program_id,
                &[
                    simple.clone(),
                    percent_tracker_pda.clone(),
                    system_program.clone(),
                ][..],
            )?;
        } else {
            return Err(SimpleProtocolError::PdaAlreadyExists.into())
        }

        if **wsol_amount_pda.lamports.borrow() == 0 { 
            initialize_wsol_amount_account(
                program_id,
                &[
                    simple.clone(),
                    wsol_amount_pda.clone(),
                    system_program.clone(),
                ][..],
            )?;
        } else {
            return Err(SimpleProtocolError::PdaAlreadyExists.into());
        }
    } else {
        return Err(SimpleProtocolError::InvalidSigner.into());
    }

    Ok(())
}
