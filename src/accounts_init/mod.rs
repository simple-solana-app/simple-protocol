use borsh::{BorshDeserialize, BorshSerialize};

pub mod program;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Tracker {
    pub increment: u8,
}