use {
    crate::{accounts_init::Tracker, common::SIMPLE_PUBKEY, error::SimpleProtocolError},
    authority::initialize_authority_account,
    percent_tracker::initialize_percent_tracker_account,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        borsh1::try_from_slice_unchecked,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    wsol_amount::{initialize_wsol_amount_account, WsolAmount},
};

pub mod authority;
pub mod percent_tracker;
pub mod wsol_amount;

pub fn init_most_program_accounts(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let simple = next_account_info(account_info_iter)?;
    let percent_tracker_pda = next_account_info(account_info_iter)?;
    let wsol_amount_pda = next_account_info(account_info_iter)?;
    let authority_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if simple.is_signer && simple.key.to_string().as_str() == SIMPLE_PUBKEY {
        initialize_percent_tracker_account(
            program_id,
            &[
                simple.clone(),
                percent_tracker_pda.clone(),
                system_program.clone(),
            ],
        )?;

        initialize_wsol_amount_account(
            program_id,
            &[
                simple.clone(),
                wsol_amount_pda.clone(),
                system_program.clone(),
            ],
        )?;

        initialize_authority_account(
            program_id,
            &[
                simple.clone(),
                authority_pda.clone(),
                system_program.clone(),
            ],
        )?;

        msg!("simple: {}, ({})", simple.key, simple.owner);

        let percent_tracker_pda_increment =
            try_from_slice_unchecked::<Tracker>(&percent_tracker_pda.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?
                .increment;

        msg!(
            "percent_tracker_pda: {}, ({}): {}",
            percent_tracker_pda.key,
            percent_tracker_pda.owner,
            percent_tracker_pda_increment
        );

        let wsol_amount_pda_amount =
            try_from_slice_unchecked::<WsolAmount>(&wsol_amount_pda.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?
                .amount;

        msg!(
            "wsol_amount_pda: {}, ({}): {}",
            wsol_amount_pda.key,
            wsol_amount_pda.owner,
            wsol_amount_pda_amount
        );

        msg!(
            "authority_pda: {}, ({})",
            authority_pda.key,
            authority_pda.owner,
        );
    } else {
        return Err(SimpleProtocolError::InvalidSigner.into());
    }

    Ok(())
}
