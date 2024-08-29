use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program::invoke_signed, program_pack::Pack, pubkey::Pubkey, rent::Rent, system_instruction::create_account, sysvar::Sysvar
};
use spl_token::state::Account;

pub fn initialize_authority_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let simple = next_account_info(account_info_iter)?;
    let authority_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let authority_seed = b"authority";
    let authority_rent = Rent::get()?;
    let authority_lamports = authority_rent.minimum_balance(0);

    let (_authority_pda_key, authority_bump_seed) =
        Pubkey::find_program_address(&[authority_seed], program_id);
        

    let ix = create_account(
        simple.key,
        authority_pda.key,
        authority_lamports,
        0,
        program_id,
    );

    invoke_signed(
        &ix,
        &[
            simple.clone(),
            authority_pda.clone(),
            system_program.clone(),
        ],
        &[&[authority_seed, &[authority_bump_seed]]],
    )?;

    Ok(())
}
