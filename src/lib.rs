mod execute;

solana_program::entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();
    let simple = next_account_info(&mut account_info_iter)?;
    let percent_tracker_pda = next_account_info(&mut account_info_iter)?;
    let wsol_amount_pda = next_account_info(&mut account_info_iter)?;
    let transfer_signer_pda = next_account_info(&mut account_info_iter)?;
    let program_simple_token_ass_account = next_account_info(&mut account_info_iter)?;
    let system_program = next_account_info(&mut account_info_iter)?;
    let simple_token = next_account_info(&mut account_info_iter)?;

                execute(
                program_id,
                simple,
                percent_tracker_pda,
                wsol_amount_pda,
                transfer_signer_pda,
                program_simple_token_ass_account,
                system_program,
            )?;
    Ok(())
}
