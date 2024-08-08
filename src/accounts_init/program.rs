use std::error::Error;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{pubkey::Pubkey, rent::Rent, sysvar::Sysvar};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct WsolBalance {
    balance: u64,
}

pub struct TransferAuthority {}

pub fn initialize_all_program_accounts(program_id: &Pubkey) {
    initialize_percent_tracker(program_id);
}

fn initialize_percent_tracker(program_id: &Pubkey) -> Result<(), Box<dyn Error>> {
    let seed = b"percent_tracker";
    let account_len: usize = std::mem::size_of::<u8>();
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    let (percent_tracker, bump) = Pubkey::find_program_address(&[seed], program_id);
    Ok(())
}