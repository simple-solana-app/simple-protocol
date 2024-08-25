use solana_program::{msg, pubkey::Pubkey};

use crate::{common::SIMPLE_PUBKEY, print_shit::print_prog_accounts, Args};

use super::{percent_tracker::initialize_percent_tracker_account, transfer_signer::initialize_transfer_signer_account, wsol_amount::initialize_wsol_amount_account};

pub fn init_prog_accounts(program_id: &Pubkey, args: Args) {
    print_prog_accounts(&args);
    if args.simple.is_signer
        && **args.percent_tracker_pda.lamports.borrow() == 0
        && args.simple.key.to_string().as_str() == SIMPLE_PUBKEY
    {
        if let Err(e) = initialize_percent_tracker_account(
            program_id,
            args.simple,
            args.percent_tracker_pda,
            args.system_program,
        ) {
            msg!("Failed to initialize percent tracker account: {:?}", e)
        }
    }

    if args.simple.is_signer
        && **args.wsol_amount_pda.lamports.borrow() == 0
        && args.simple.key.to_string().as_str() == SIMPLE_PUBKEY
    {
        if let Err(e) = initialize_wsol_amount_account(
            program_id,
            args.simple,
            args.wsol_amount_pda,
            args.system_program,
        ) {
            msg!("Failed to initialize WSOL balance account: {:?}", e)
        }
    }

    if args.simple.is_signer
        && **args.transfer_signer_pda.lamports.borrow() == 0
        && args.simple.key.to_string().as_str() == SIMPLE_PUBKEY
    {
        if let Err(e) = initialize_transfer_signer_account(
            program_id,
            args.simple,
            args.transfer_signer_pda,
            args.system_program,
        ) {
            msg!("Failed to initialize Transfer Signer account: {:?}", e)
        }
    }
}