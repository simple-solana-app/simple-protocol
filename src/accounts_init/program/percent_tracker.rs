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
        system_instruction,
        sysvar::Sysvar,
    },
};

pub fn initialize_percent_tracker_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let simple = next_account_info(account_info_iter)?;
    let percent_tracker_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let seed = b"percent_tracker";
    let account_len: usize = std::mem::size_of::<u8>();
    let rent = Rent::get().unwrap();
    let rent_lamports = rent.minimum_balance(account_len);

    let (_percent_tracker_address, bump_seed) = Pubkey::find_program_address(&[seed], program_id);

    invoke_signed(
        &system_instruction::create_account(
            simple.key,
            percent_tracker_pda.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            simple.clone(),
            percent_tracker_pda.clone(),
            system_program.clone(),
        ],
        &[&[seed, &[bump_seed]]],
    )?;

    // Handle the result of `serialize`
    let mut account_data = try_from_slice_unchecked::<Tracker>(&percent_tracker_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;
    account_data.increment = 0;
    account_data
        .serialize(&mut &mut percent_tracker_pda.data.borrow_mut()[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(())
}
