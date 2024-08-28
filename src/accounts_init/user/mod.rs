use {
    super::Tracker,
    crate::error::SimpleProtocolError,
    solana_program::{
        account_info::{self, next_account_info, AccountInfo},
        borsh1::try_from_slice_unchecked,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    user_claim_tracker::initialize_user_claim_tracker_account,
};

mod user_claim_tracker;

pub fn init_user_accounts(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user = next_account_info(account_info_iter)?;
    let user_claim_tracker_pda = next_account_info(account_info_iter)?;
    //let user_simple_ata = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if user.is_signer {
        if **user_claim_tracker_pda.lamports.borrow() == 0 {
            initialize_user_claim_tracker_account(
                program_id,
                &[
                    user.clone(),
                    user_claim_tracker_pda.clone(),
                    system_program.clone(),
                ][..],
            )?;
        } else {
            return Err(SimpleProtocolError::AccountAlreadyExists.into());
        }

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
        )
    } else {
        return Err(SimpleProtocolError::InvalidSigner.into());
    }

    Ok(())
}
