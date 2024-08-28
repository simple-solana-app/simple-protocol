use {
    super::Tracker, crate::{
        common::{SIMPLE_MINT, TOKEN_PROGRAM_ID},
        error::SimpleProtocolError,
    }, solana_program::{
        account_info::{self, next_account_info, AccountInfo}, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult, msg, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey
    }, spl_associated_token_account::get_associated_token_address, spl_token_2022::state::Account, user_claim_tracker::initialize_user_claim_tracker_account, user_simple_ata::initialize_user_simple_ata
};

mod user_claim_tracker;
mod user_simple_ata;

pub fn init_user_accounts(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user = next_account_info(account_info_iter)?;
    let user_claim_tracker_pda = next_account_info(account_info_iter)?;
    let user_simple_ata = next_account_info(account_info_iter)?;
    let simple_token_mint = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if user.is_signer
        && simple_token_mint.key.to_string().as_str() == SIMPLE_MINT
        && user_simple_ata.key == &get_associated_token_address(user.key, simple_token_mint.key)
        && token_program.key.to_string().as_str() == TOKEN_PROGRAM_ID
    {
        // initialize_user_claim_tracker_account(
        //     program_id,
        //     &[
        //         user.clone(),
        //         user_claim_tracker_pda.clone(),
        //         system_program.clone(),
        //     ][..],
        // )?;

        let user_claim_tracker_pda_increment =
            try_from_slice_unchecked::<Tracker>(&user_claim_tracker_pda.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?
                .increment;

        msg!("user: {}, ({})", user.key, user.owner);
        msg!(
            "user_claim_tracker_pda: {}, ({}): {}",
            user_claim_tracker_pda.key,
            user_claim_tracker_pda.owner,
            user_claim_tracker_pda_increment
        );

        initialize_user_simple_ata(
            &[
                user.clone(),
                user_simple_ata.clone(),
                simple_token_mint.clone(),
                token_program.clone(),
                associated_token_program.clone(),
                system_program.clone(),
            ][..],
        )?;

        let user_simple_ata_amount = Account::unpack(&user_simple_ata.data.borrow()).unwrap().amount;

        msg!(
            "user_simple_ata {} ({}): {} simple",
            user_simple_ata.key,
            user_simple_ata.owner,
            user_simple_ata_amount
        )
    } else {
        return Err(SimpleProtocolError::InvalidSigner.into());
    }

    Ok(())
}
