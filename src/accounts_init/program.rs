use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh1::try_from_slice_unchecked,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::accounts_init::Tracker;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct WsolBalance {
    pub balance: u64,
}

pub struct TransferSigner {}

pub fn initialize_all_program_accounts<'a>(
    program_id: &Pubkey,
    simple: &'a AccountInfo<'a>,
    percent_tracker_pda: &'a AccountInfo<'a>,
    wsol_balance_pda: &'a AccountInfo<'a>,
    transfer_signer_pda: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,

) {
    if let Err(e) =
        initialize_percent_tracker_account(program_id, simple, percent_tracker_pda, system_program)
    {
        msg!("Failed to initialize percent tracker account: {:?}", e)
    }

    if let Err(e) =
        initialize_wsol_balance_account(program_id, simple, wsol_balance_pda, system_program)
    {
        msg!("Failed to initialize WSOL balance account: {:?}", e)
    }

    if let Err(e) =
        initialize_transfer_signer_account(program_id, simple, transfer_signer_pda, system_program)
    {
        msg!("Failed to initialize Transfer Signer account: {:?}", e)
    }
}

fn initialize_percent_tracker_account<'a>(
    program_id: &Pubkey,
    simple: &'a AccountInfo<'a>,
    percent_tracker_pda: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let seed = b"percent_tracker_pda";
    let account_len: usize = std::mem::size_of::<u8>();
    let rent = Rent::get().unwrap();
    let rent_lamports = rent.minimum_balance(account_len);

    let (_percent_tracker_address, bump_seed) = Pubkey::find_program_address(&[seed], program_id);

    // Handle the result of `invoke_signed`
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

fn initialize_wsol_balance_account<'a>(
    program_id: &Pubkey,
    simple: &'a AccountInfo<'a>,
    wsol_balance_pda: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let seed = b"wsol_balance_pda";
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
    let mut account_data = try_from_slice_unchecked::<WsolBalance>(&wsol_balance_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;
    account_data.balance = 0;
    account_data
        .serialize(&mut &mut wsol_balance_pda.data.borrow_mut()[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;
    Ok(())
}

fn initialize_transfer_signer_account<'a>(
    program_id: &Pubkey,
    simple: &'a AccountInfo<'a>,
    tranfer_signer_pda: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let seed = b"transfer_signer_pda";
    let rent = Rent::get().unwrap();
    let rent_lamports = rent.minimum_balance(0);

    let (_transfer_signer_address, bump_seed) = Pubkey::find_program_address(&[seed], program_id);

    // Handle the result of `invoke_signed`
    invoke_signed(
        &system_instruction::create_account(
            simple.key,
            tranfer_signer_pda.key,
            rent_lamports,
            0,
            program_id,
        ),
        &[
            simple.clone(),
            tranfer_signer_pda.clone(),
            system_program.clone(),
        ],
        &[&[seed, &[bump_seed]]],
    )?;

    Ok(())
}