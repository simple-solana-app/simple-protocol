use accounts_init::{user::{initialize_user_accounts, initialize_user_claim_tracker_account, initialize_user_simple_token_account}, program::initialize_all_program_accounts};
use instructions::SimpleInstructions;
use execute::execute;


use solana_program::{
    entrypoint,
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
};

pub mod instructions;
pub mod execute;
pub mod accounts_init;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SimpleInstructions::unpack(instruction_data)?;

    match instruction {
        SimpleInstructions::InitRequiredProgramAccounts => {
            initialize_all_program_accounts(program_id);
        }
        SimpleInstructions::InitRequiredUserAccountsAndExecute {
            has_claim_account,
            has_simple_token_account,
        } => match (has_claim_account, has_simple_token_account) {
            (false, true) => initialize_user_claim_tracker_account(),
            (true, false) => initialize_user_simple_token_account(),
            (false, false) => initialize_user_accounts(),
            (true, true) => execute(),
        },
        SimpleInstructions::Execute => {
            execute();
        }
    }
    Ok(())
}
