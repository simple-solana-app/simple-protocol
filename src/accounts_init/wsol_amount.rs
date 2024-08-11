use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct WsolAmount {
    pub amount: u64,
}

pub fn initialize_wsol_amount_account<'a>(
    program_id: &Pubkey,
    simple: &'a AccountInfo<'a>,
    wsol_balance_pda: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let seed = b"wsol_amount_pda";
    let account_len: usize = std::mem::size_of::<u64>();
    let rent = Rent::get().unwrap();
    let rent_lamports = rent.minimum_balance(account_len);

    let (_wsol_balance_address, bump_seed) = Pubkey::find_program_address(&[seed], program_id);

    // Handle the result of `invoke_signed`
    invoke_signed(
        &system_instruction::create_account(
            simple.key,
            wsol_balance_pda.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            simple.clone(),
            wsol_balance_pda.clone(),
            system_program.clone(),
        ],
        &[&[seed, &[bump_seed]]],
    )?;

    // Handle the result of `serialize`
    let mut account_data = try_from_slice_unchecked::<WsolAmount>(&wsol_balance_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;
    account_data.amount = 0;
    account_data
        .serialize(&mut &mut wsol_balance_pda.data.borrow_mut()[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;
    Ok(())
}
