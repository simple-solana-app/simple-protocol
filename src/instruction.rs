use solana_program::program_error::ProgramError;

pub enum SimpleInstruction {
    InitMostProgramAccounts,
    Execute,
}

impl SimpleInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let tag = input[0];
        Ok(match tag {
            0 => Self::InitMostProgramAccounts,
            1 => Self::Execute,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
