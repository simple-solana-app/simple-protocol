pub fn execute<'a>(
    program_id: &Pubkey,
    simple: &'a AccountInfo<'a>,
    percent_tracker_pda: &'a AccountInfo<'a>,
    wsol_amount_pda: &'a AccountInfo<'a>,
    transfer_signer: &'a AccountInfo<'a>,
    program_simple_token_ass_account: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,
) -> Result<(), ProgramError> {

    if simple.is_signer && **percent_tracker_pda.lamports.borrow() == 0 {
        if let Err(e) =
            initialize_percent_tracker_account(program_id, simple, percent_tracker_pda, system_program)
        {
            msg!("Failed to initialize percent tracker account: {:?}", e)
        }
    }

    if simple.is_signer && **wsol_amount_pda.lamports.borrow() == 0 {
        if let Err(e) =
            initialize_wsol_amount_account(program_id, simple, wsol_amount_pda, system_program)
        {
            msg!("Failed to initialize WSOL balance account: {:?}", e)
        }
    }

    if simple.is_signer && **transfer_signer_pda.lamports.borrow() == 0 {
        if let Err(e) =
            initialize_transfer_signer_account(program_id, simple, transfer_signer_pda, system_program)
        {
            msg!("Failed to initialize Transfer Signer account: {:?}", e)
        }
    }

    let wsol_balance_account_data = try_from_slice_unchecked::<WsolAmount>(&wsol_amount_pda.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let program_simple_token_ass_account_data = TokenAccount::unpack(&program_simple_token_ass_account.data.borrow())?;
    
    msg!(
        "Percent Tracker: {} ({}): {:?}",
        percent_tracker_pda.key,
        percent_tracker_pda.owner,
        percent_tracker_pda.data
    );
    msg!(
        "WSOL Balance: {} ({}): {:?}",
        wsol_amount_pda.key,
        wsol_amount_pda.owner,
        wsol_balance_account_data.balance
    );
    msg!(
        "Program's simple ass token acc: {} ({}): {:?}",
        program_simple_token_ass_account.key,
        program_simple_token_ass_account.owner,
        program_simple_token_ass_account_data.amount,
    );

    Ok(())
}

