use borsh::{BorshDeserialize, BorshSerialize};

pub mod program;
pub mod user;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Tracker {
    pub increment: u8,
}
