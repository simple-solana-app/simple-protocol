use execute::ExecuteArgs;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

mod accounts_init;
mod common;
mod execute;
mod simple_errors;
mod transfer_simple;
mod print_statements;
use crate::execute::execute;
use crate::print_statements::print_statements;

solana_program::entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    _instruction_data: &[u8],
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let args = ExecuteArgs {
        simple: next_account_info(&mut account_info_iter)?,

        user: next_account_info(&mut account_info_iter)?,
        user_claim_tracker_pda: next_account_info(&mut account_info_iter)?,
        user_raydium_pool_lp_ass_token_account: next_account_info(&mut account_info_iter)?,
        user_simple_token_ass_account: next_account_info(&mut account_info_iter)?,

        percent_tracker_pda: next_account_info(&mut account_info_iter)?,
        wsol_amount_pda: next_account_info(&mut account_info_iter)?,
        transfer_signer_pda: next_account_info(&mut account_info_iter)?,
        program_simple_token_ass_account: next_account_info(&mut account_info_iter)?,

        simple_token_mint_account: next_account_info(&mut account_info_iter)?,
        raydium_pool_wsol_token_account: next_account_info(&mut account_info_iter)?,
        raydium_pool_lp_token_mint_account: next_account_info(&mut account_info_iter)?,

        system_program: next_account_info(&mut account_info_iter)?,
        token_program: next_account_info(&mut account_info_iter)?,
        associated_token_program: next_account_info(&mut account_info_iter)?,

    };

    print_statements(&args);

    execute(program_id, args)?;

    Ok(())
}
