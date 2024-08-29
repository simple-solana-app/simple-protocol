use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program::invoke,
    },
    spl_associated_token_account::instruction::create_associated_token_account_idempotent,
};

pub fn initialize_user_simple_ata(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user = next_account_info(account_info_iter)?;
    let user_simple_ata = next_account_info(account_info_iter)?;
    let simple_token_mint = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let ix = create_associated_token_account_idempotent(
        user.key,
        user.key,
        simple_token_mint.key,
        token_program.key,
    );

    invoke(
        &ix,
        &[
            user.clone(),
            user_simple_ata.clone(),
            user.clone(),
            simple_token_mint.clone(),
            token_program.clone(),
            associated_token_program.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}
