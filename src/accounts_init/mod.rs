use borsh::{BorshSerialize, BorshDeserialize};

pub mod program;
pub mod user;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Tracker {
    increment: u8,
}