use {
    solana_program::{
        account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::{invoke, invoke_signed}, program_pack::Pack, pubkey::Pubkey, rent::Rent, system_instruction::create_account, sysvar::Sysvar
    },
    spl_token_2022::{
        instruction::initialize_account,
        state::Account,
    },
};

pub fn initialize_program_simple_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let simple = next_account_info(account_info_iter)?;
    let authority_pda = next_account_info(account_info_iter)?;
    let program_simple_pda = next_account_info(account_info_iter)?;
    let simple_token_mint = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let authority_seed = b"authority";
    let authority_rent = Rent::get()?;
    let authority_lamports = authority_rent.minimum_balance(0);

    let (_authority_pda_key, bump_seed) =
        Pubkey::find_program_address(&[authority_seed], program_id);

    let ix = create_account(simple.key, authority_pda.key, authority_lamports, 0, program_id);

    invoke_signed(
        &ix,
        &[
            simple.clone(),
            authority_pda.clone(),
            system_program.clone(),
        ],
        &[&[authority_seed, &[bump_seed]]],
    )?;

    let simple_seed = b"simple";
    let simple_rent = Rent::get()?;
    let simple_lamports = simple_rent.minimum_balance(Account::LEN);

    let (_program_simple_pda_key, bump_seed) =
        Pubkey::find_program_address(&[simple_seed], program_id);

    // let ix_create_account = create_account(
    //     simple.key,
    //     authority_pda.key,
    //     simple_lamports,
    //     Account::LEN as u64,
    //     authority_pda.key,
    // );

    // invoke_signed(
    //     &ix_create_account,
    //     &[simple.clone(), system_program.clone()],
    //     &[&[simple_seed, &[bump_seed]]],
    // )?;

    // let ix_initialize_account = initialize_account(
    //     token_program.key,
    //     program_simple_pda.key,
    //     simple_token_mint.key,
    //     authority_pda.key,
    // )?;

    // invoke(
    //     &ix_initialize_account,
    //     &[
    //         simple.clone(),
    //         authority_pda.clone(),
    //         simple_token_mint.clone(),
    //         token_program.clone(),
    //     ],
    // )?;

    Ok(())
}
