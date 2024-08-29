use {
    borsh::{BorshDeserialize, BorshSerialize},
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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct WsolAmount {
    pub amount: u64,
}

pub fn initialize_wsol_amount_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let simple = next_account_info(account_info_iter)?;
    let wsol_amount_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let seed = b"wsol_amount";
    let account_len: usize = std::mem::size_of::<u64>();
    let rent = Rent::get().unwrap();
    let rent_lamports = rent.minimum_balance(account_len);

    let (_wsol_balance_address, bump_seed) = Pubkey::find_program_address(&[seed], program_id);

    invoke_signed(
        &create_account(
            simple.key,
            wsol_amount_pda.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            simple.clone(),
            wsol_amount_pda.clone(),
            system_program.clone(),
        ],
        &[&[seed, &[bump_seed]]],
    )?;

    // Handle the result of `serialize`
    let mut account_data = try_from_slice_unchecked::<WsolAmount>(&wsol_amount_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;
    account_data.amount = 0;
    account_data
        .serialize(&mut &mut wsol_amount_pda.data.borrow_mut()[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;
    Ok(())
}
