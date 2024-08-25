use instructions::SimpleInstructions;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

mod print_shit;
mod common;
mod instructions;
mod accounts_init;
mod execute;
mod transfer_simple;
mod simple_errors;
use crate::accounts_init::program::init_prog_accounts::init_prog_accounts;
use crate::execute::execute;

pub struct Args<'a> {
    pub simple: &'a AccountInfo<'a>,

    pub user: &'a AccountInfo<'a>,
    pub user_claim_tracker_pda: &'a AccountInfo<'a>,
    pub user_simple_pool_lp_ass_token_account: &'a AccountInfo<'a>,
    pub user_simple_token_ass_account: &'a AccountInfo<'a>,

    pub percent_tracker_pda: &'a AccountInfo<'a>,
    pub wsol_amount_pda: &'a AccountInfo<'a>,
    pub transfer_signer_pda: &'a AccountInfo<'a>,
    pub program_simple_token_ass_account: &'a AccountInfo<'a>,

    pub simple_token_mint_account: &'a AccountInfo<'a>,
    pub simple_pool_wsol_token_account: &'a AccountInfo<'a>,
    pub simple_pool_lp_token_mint_account: &'a AccountInfo<'a>,

    pub system_program: &'a AccountInfo<'a>,
    pub token_program: &'a AccountInfo<'a>,
    pub associated_token_program: &'a AccountInfo<'a>,
}


solana_program::entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SimpleInstructions::unpack(instruction_data)?;

    let mut account_info_iter = accounts.iter();

    let args = Args {
        simple: next_account_info(&mut account_info_iter)?,

        user: next_account_info(&mut account_info_iter)?,
        user_claim_tracker_pda: next_account_info(&mut account_info_iter)?,
        user_simple_pool_lp_ass_token_account: next_account_info(&mut account_info_iter)?,
        user_simple_token_ass_account: next_account_info(&mut account_info_iter)?,

        percent_tracker_pda: next_account_info(&mut account_info_iter)?,
        wsol_amount_pda: next_account_info(&mut account_info_iter)?,
        transfer_signer_pda: next_account_info(&mut account_info_iter)?,
        program_simple_token_ass_account: next_account_info(&mut account_info_iter)?,

        simple_token_mint_account: next_account_info(&mut account_info_iter)?,
        simple_pool_wsol_token_account: next_account_info(&mut account_info_iter)?,
        simple_pool_lp_token_mint_account: next_account_info(&mut account_info_iter)?,

        system_program: next_account_info(&mut account_info_iter)?,
        token_program: next_account_info(&mut account_info_iter)?,
        associated_token_program: next_account_info(&mut account_info_iter)?,
    };

    match instruction {
        SimpleInstructions::InitRequiredProgramAccounts => {
            init_prog_accounts(program_id, args);
        }
        SimpleInstructions::InitRequiredUserAccountsAndExecute => {}
        SimpleInstructions::Execute => {
            execute(program_id, args)?;
        }
    }

    Ok(())
}
