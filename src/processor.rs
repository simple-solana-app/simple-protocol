use {
    crate::{
        accounts_init::program::init_prog_accounts, instruction::SimpleInstruction,
        print_shit::print_shit,
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
                print_shit(accounts)
            }
            SimpleInstruction::InitRequiredUserAccountsAndExecute => print_shit(accounts),
            SimpleInstruction::Execute => print_shit(accounts),
        }
    }
}
