use solana_program::{
    account_info::AccountInfo, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey,
    rent::Rent, system_instruction, sysvar::Sysvar,
};

pub fn initialize_transfer_signer_account<'a>(
    program_id: &Pubkey,
    simple: &'a AccountInfo<'a>,
    transfer_signer_pda: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let seed = b"transfer_signer_pda";
    let rent = Rent::get().unwrap();
    let rent_lamports = rent.minimum_balance(0);

    let (_transfer_signer_address, bump_seed) = Pubkey::find_program_address(&[seed], program_id);

    invoke_signed(
        &system_instruction::create_account(
            simple.key,
            transfer_signer_pda.key,
            rent_lamports,
            0,
            program_id,
        ),
        &[
            simple.clone(),
            transfer_signer_pda.clone(),
            system_program.clone(),
        ],
        &[&[seed, &[bump_seed]]],
    )?;

    Ok(())
}
