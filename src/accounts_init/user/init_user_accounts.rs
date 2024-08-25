pub fn init_user_accounts() {
        print_shit(&args);

        if args.user.is_signer && **args.user_claim_tracker_pda.lamports.borrow() == 0 {
        if let Err(e) = initialize_user_claim_tracker_account(
            program_id,
            args.user,
            args.user_claim_tracker_pda,
            args.system_program,
        ) {
            msg!("Failed to initialize User Claim account: {:?}", e)
        }
    }

}