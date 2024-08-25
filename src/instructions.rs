use solana_program::program_error::ProgramError;

pub enum SimpleInstructions {
    InitRequiredProgramAccounts,
    InitRequiredUserAccountsAndExecute,
    Execute,
}

impl SimpleInstructions {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        if instruction_data.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let tag = instruction_data[0];
        Ok(match tag {
            0 => Self::InitRequiredProgramAccounts,
            1 => Self::InitRequiredUserAccountsAndExecute,
            2 => Self::Execute,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
