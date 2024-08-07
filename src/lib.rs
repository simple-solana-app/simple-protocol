use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SimpleInstructions::unpack(instruction_data)?;

    match instruction {
        SimpleInstructions::InitRequiredProgramAccounts => {
            
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

pub fn execute() {}

pub fn initialize_user_claim_tracker_account() {}

pub fn initialize_user_simple_token_account() {}

pub fn initialize_user_accounts() {
    initialize_user_claim_tracker_account();
    initialize_user_simple_token_account();
}

pub enum SimpleInstructions {
    InitRequiredProgramAccounts,
    InitRequiredUserAccountsAndExecute {
        has_claim_account: bool,
        has_simple_token_account: bool,
    },
    Execute,
}

#[derive(BorshDeserialize)]
struct SimpleInstructionsPayload {
    has_claim_account: bool,
    has_simple_token_account: bool,
}

impl SimpleInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, user_info_raw) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        let user_info = SimpleInstructionsPayload::try_from_slice(user_info_raw).unwrap();

        Ok(match variant {
            0 => Self::InitRequiredProgramAccounts,
            1 => Self::InitRequiredUserAccountsAndExecute {
                has_claim_account: user_info.has_claim_account,
                has_simple_token_account: user_info.has_simple_token_account,
            },
            2 => Self::Execute,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
