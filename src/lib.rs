use accounts_init::{
    program::initialize_all_program_accounts,
    user::{
        initialize_user_accounts, initialize_user_claim_tracker_account,
        initialize_user_simple_token_account,
    },
    Tracker,
};
use borsh::BorshSerialize;
use execute::execute;
use instructions::SimpleInstructions;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh1::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub mod accounts_init;
pub mod execute;
pub mod instructions;

solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let simple = next_account_info(account_info_iter)?;
    let percent_tracker_pda = next_account_info(account_info_iter)?;

    let system_program = next_account_info(account_info_iter)?;

    let instruction = SimpleInstructions::unpack(instruction_data)?;

    match instruction {
        SimpleInstructions::InitRequiredProgramAccounts => {
            initialize_all_program_accounts(
                program_id,
                simple,
                percent_tracker_pda,
                system_program,
            );
        }
        SimpleInstructions::InitRequiredUserAccountsAndExecute {
            has_claim_account,
            has_simple_token_account,
        } => match (has_claim_account, has_simple_token_account) {
            (false, true) => initialize_user_claim_tracker_account(),
            (true, false) => initialize_user_simple_token_account(),
            (false, false) => initialize_user_accounts(),
            (true, true) => execute(), // won't ever used by ui
        },
        SimpleInstructions::Execute => {
            execute();
        }
    }
    Ok(())
}
