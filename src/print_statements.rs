use solana_program::msg;
use crate::execute::ExecuteArgs;

pub fn print_statements(args: &ExecuteArgs) {
    msg!("simple: {} ({})", args.simple.key, args.simple.owner);
    msg!("user: {} ({})", args.user.key, args.user.owner);
    msg!("user_claim_tracker_pda: {} ({})", args.user_claim_tracker_pda.key, args.user_claim_tracker_pda.owner);
    msg!("user_raydium_pool_lp_ass_token_account: {} ({})", args.user_raydium_pool_lp_ass_token_account.key, args.user_raydium_pool_lp_ass_token_account.owner);
    msg!("user_simple_token_ass_account: {} ({})", args.user_simple_token_ass_account.key, args.user_simple_token_ass_account.owner);
    msg!("percent_tracker_pda: {} ({})", args.percent_tracker_pda.key, args.percent_tracker_pda.owner);
    msg!("wsol_amount_pda: {} ({})", args.wsol_amount_pda.key, args.wsol_amount_pda.owner);
    msg!("transfer_signer_pda: {} ({})", args.transfer_signer_pda.key, args.transfer_signer_pda.owner);
    msg!("program_simple_token_ass_account: {} ({})", args.program_simple_token_ass_account.key, args.program_simple_token_ass_account.owner);
    msg!("simple_token_mint_account: {} ({})", args.simple_token_mint_account.key, args.simple_token_mint_account.owner);
    msg!("raydium_pool_wsol_token_account: {} ({})", args.raydium_pool_wsol_token_account.key, args.raydium_pool_wsol_token_account.owner);
    msg!("raydium_pool_lp_token_mint_account: {} ({})", args.raydium_pool_lp_token_mint_account.key, args.raydium_pool_lp_token_mint_account.owner);
    msg!("system_program: {} ({})", args.system_program.key, args.system_program.owner);
    msg!("token_program: {} ({})", args.token_program.key, args.token_program.owner);
    msg!("associated_token_program: {} ({})", args.associated_token_program.key, args.associated_token_program.owner);
}
