use {
    crate::{accounts_init::Tracker, common::SIMPLE_PUBKEY, error::SimpleProtocolError},
    percent_tracker::initialize_percent_tracker_account,
    program_simple_account::initialize_program_simple_account,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        borsh1::try_from_slice_unchecked,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    },
    spl_token_2022::state::Account,
    wsol_amount::{initialize_wsol_amount_account, WsolAmount},
};

pub mod percent_tracker;
pub mod program_simple_account;
pub mod wsol_amount;

pub fn init_prog_accounts(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let simple = next_account_info(account_info_iter)?;
    let percent_tracker_pda = next_account_info(account_info_iter)?;
    let wsol_amount_pda = next_account_info(account_info_iter)?;
    let authority_pda = next_account_info(account_info_iter)?;
    let program_simple_pda = next_account_info(account_info_iter)?;
    let simple_token_mint = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;
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

        initialize_program_simple_account(
            program_id,
            &[
                simple.clone(),
                authority_pda.clone(),
                simple_token_mint.clone(),
                token_program.clone(),
                associated_token_program.clone(),
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

        // let program_simple_pda_amount = Account::unpack(&program_simple_pda.data.borrow())
        //     .unwrap()
        //     .amount;

        msg!(
            "program_simple_account: {}, ({}):",
            program_simple_pda.key,
            program_simple_pda.owner,
            //program_simple_pda_amount
        );
    } else {
        return Err(SimpleProtocolError::InvalidSigner.into());
    }

    Ok(())
}
