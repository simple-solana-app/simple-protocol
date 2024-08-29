use {
    crate::accounts_init::Tracker,
    borsh::BorshSerialize,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        borsh1::try_from_slice_unchecked,
        entrypoint::ProgramResult,
        program::invoke_signed,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction::create_account,
        sysvar::Sysvar,
    },
};

pub fn initialize_user_claim_tracker_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user = next_account_info(account_info_iter)?;
    let user_claim_tracker_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let seed = b"user_claim_tracker";
    let account_len: usize = std::mem::size_of::<u8>();
    let rent = Rent::get().unwrap();
    let rent_lamports = rent.minimum_balance(account_len);

    let (_user_claim_tracker_address, bump_seed) =
        Pubkey::find_program_address(&[seed, user.key.as_ref()], program_id);

    invoke_signed(
        &create_account(
            user.key,
            user_claim_tracker_pda.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            user.clone(),
            user_claim_tracker_pda.clone(),
            system_program.clone(),
        ],
        &[&[seed, user.key.as_ref(), &[bump_seed]]],
    )?;

    let mut account_data =
        try_from_slice_unchecked::<Tracker>(&user_claim_tracker_pda.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;
    account_data.increment = 0;
    account_data
        .serialize(&mut &mut user_claim_tracker_pda.data.borrow_mut()[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(())
}
