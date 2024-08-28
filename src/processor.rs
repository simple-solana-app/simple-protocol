use {
    crate::{
        accounts_init::{program::init_prog_accounts, user::init_user_accounts},
        instruction::SimpleInstruction,
    },
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

pub struct Processor {}
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = SimpleInstruction::unpack(input)?;
        match instruction {
            SimpleInstruction::InitRequiredProgramAccounts => {
                init_prog_accounts(program_id, accounts)?;
                Ok(())
            }
            SimpleInstruction::InitRequiredUserAccountsAndExecute => {
                init_user_accounts(program_id, accounts)?;
                Ok(())
            }
            SimpleInstruction::Execute => Ok(()),
        }
    }
}
