use accounts_init::{
    program::initialize_all_program_accounts,
    user::{
        initialize_user_accounts, initialize_user_claim_tracker_account,
        initialize_user_simple_ass_token_account,
    },
};
use execute::execute;
use instructions::SimpleInstructions;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub mod accounts_init;
pub mod execute;
pub mod instructions;

solana_program::entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();
    let simple = next_account_info(&mut account_info_iter)?;
    let percent_tracker_pda = next_account_info(&mut account_info_iter)?;
    let wsol_balance_pda = next_account_info(&mut account_info_iter)?;
    let system_program = next_account_info(&mut account_info_iter)?;

    let instruction = SimpleInstructions::unpack(instruction_data)?;

    match instruction {
        SimpleInstructions::InitRequiredProgramAccounts => {
            initialize_all_program_accounts(
                program_id,
                simple,
                percent_tracker_pda,
                wsol_balance_pda,
                system_program,
            );
        }
        SimpleInstructions::InitRequiredUserAccountsAndExecute {
            has_claim_account,
            has_simple_token_account,
        } => match (has_claim_account, has_simple_token_account) {
            (false, true) => initialize_user_claim_tracker_account(),
            (true, false) => initialize_user_simple_ass_token_account(),
            (false, false) => initialize_user_accounts(),
            (true, true) => execute(percent_tracker_pda, wsol_balance_pda)?, // won't ever be used by UI
        },
        SimpleInstructions::Execute => {
            execute(percent_tracker_pda, wsol_balance_pda);
        }
    }
    Ok(())
}
