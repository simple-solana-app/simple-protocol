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
    Ok(())
}

pub enum SimpleInstructions {
    InitRequiredProgramAccounts,
    InitRequiredUserAccountsAndExecute {
        has_claim_account: bool,
        has_ass_simple_token_account: bool,
    },
    Execute,
}

#[derive(BorshDeserialize)]
struct SimpleInstructionsPayload {
    has_claim_account: bool,
    has_ass_simple_token_account: bool,
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
                has_ass_simple_token_account: user_info.has_ass_simple_token_account,
            },
            2 => Self::Execute,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
