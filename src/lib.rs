use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

mod accounts_init;
mod common;
mod execute;
mod simple_errors;
use crate::execute::execute;

solana_program::entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    _instruction_data: &[u8],
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let simple = next_account_info(&mut account_info_iter)?;
    let user = next_account_info(&mut account_info_iter)?;
    let percent_tracker_pda = next_account_info(&mut account_info_iter)?;
    let wsol_amount_pda = next_account_info(&mut account_info_iter)?;
    let transfer_signer_pda = next_account_info(&mut account_info_iter)?;
    let user_claim_tracker_pda = next_account_info(&mut account_info_iter)?;
    let program_simple_token_ass_account = next_account_info(&mut account_info_iter)?;
    let user_simple_token_ass_account = next_account_info(&mut account_info_iter)?;
    let system_program = next_account_info(&mut account_info_iter)?;
    let simple_token_mint_account = next_account_info(&mut account_info_iter)?;
    let raydium_pool_wsol_token_account = next_account_info(&mut account_info_iter)?;
    let raydium_pool_lp_token_mint_account = next_account_info(&mut account_info_iter)?;
    let user_raydium_pool_lp_ass_token_account = next_account_info(&mut account_info_iter)?;

    execute(
        program_id,
        simple,
        user,
        percent_tracker_pda,
        wsol_amount_pda,
        transfer_signer_pda,
        user_claim_tracker_pda,
        program_simple_token_ass_account,
        user_simple_token_ass_account,
        system_program,
        simple_token_mint_account,
        raydium_pool_wsol_token_account,
        raydium_pool_lp_token_mint_account,
        user_raydium_pool_lp_ass_token_account,
    )?;

    Ok(())
}
