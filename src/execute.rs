// use crate::{
//     accounts_init::program::{tracker::Tracker, wsol_amount::WsolAmount},
//     common::{
//         PROGRAM_SIMPLE_ASS_TOKEN_ACCOUNT_INITIAL_AMOUNT, SIMPLE_LP_MINT, SIMPLE_MINT,
//         SIMPLE_POOL_WSOL_TOKEN_ACCOUNT_PUBKEY, SIMPLE_PUBKEY,
//     },
//     error::SimpleProtocolError,
//     print_shit::print_execute_args,
//     transfer_simple::transfer_simple,
//     Args,
// };
// use borsh::BorshSerialize;
// use solana_program::{
//     account_info::AccountInfo, borsh1::try_from_slice_unchecked, msg,
//     native_token::LAMPORTS_PER_SOL, program_error::ProgramError, pubkey::Pubkey,
// };
// use spl_associated_token_account::get_associated_token_address;
// use spl_token::{
//     solana_program::program_pack::Pack,
//     state::{Account as TokenAccount, Mint},
// };

// pub fn execute(program_id: &Pubkey, args: Args) -> Result<(), ProgramError> {
//     print_execute_args(&args);

//     let user_simple_ata =
//         get_associated_token_address(args.user.key, args.simple_token_mint_account.key);
//     let user_raydium_pool_lp_ata =
//         get_associated_token_address(args.user.key, args.simple_pool_lp_token_mint_account.key);

//     if args.user.is_signer
//         && **args.user_simple_token_ass_account.lamports.borrow() != 0
//         && args.user_simple_token_ass_account.key == &user_simple_ata
//         && args.simple_pool_wsol_token_account.key.to_string().as_str()
//             == SIMPLE_POOL_WSOL_TOKEN_ACCOUNT_PUBKEY
//         && args.user_simple_pool_lp_ass_token_account.key == &user_raydium_pool_lp_ata
//         && args.simple_token_mint_account.key.to_string().as_str() == SIMPLE_MINT
//         && args
//             .simple_pool_lp_token_mint_account
//             .key
//             .to_string()
//             .as_str()
//             == SIMPLE_LP_MINT
//     {
//         let program_simple_token_ass_account_amount =
//             TokenAccount::unpack(&args.program_simple_token_ass_account.data.borrow())
//                 .unwrap()
//                 .amount;

//         let mut percent_tracker_account_data =
//             try_from_slice_unchecked::<Tracker>(&args.percent_tracker_pda.data.borrow())
//                 .map_err(|_| ProgramError::InvalidAccountData)?;

//         let total_drainable_simple = program_simple_token_ass_account_amount as f64
//             * percent_tracker_account_data.increment as f64
//             / 100.0;

//         if (program_simple_token_ass_account_amount as f64)
//             < (PROGRAM_SIMPLE_ASS_TOKEN_ACCOUNT_INITIAL_AMOUNT - total_drainable_simple)
//         {
//             msg!("Max drainable simple has already been claimed for this round");
//             return Err(SimpleProtocolError::MaxSimpleDrained.into());
//         }

//         let raydium_pool_wsol_token_account_amount =
//             TokenAccount::unpack(&args.simple_pool_wsol_token_account.data.borrow())
//                 .unwrap()
//                 .amount;

//         let total_wsol_in_pools = raydium_pool_wsol_token_account_amount; // future simple native pool impl?

//         let mut wsol_amount_account_data =
//             try_from_slice_unchecked::<WsolAmount>(&args.wsol_amount_pda.data.borrow())
//                 .map_err(|_| ProgramError::InvalidAccountData)?;

//         if total_wsol_in_pools >= wsol_amount_account_data.amount + 50_000_000_000 {
//             wsol_amount_account_data.amount = total_wsol_in_pools;
//             wsol_amount_account_data
//                 .serialize(&mut &mut args.wsol_amount_pda.data.borrow_mut()[..])
//                 .map_err(|_| ProgramError::InvalidAccountData)?;

//             percent_tracker_account_data.increment += 1;
//             percent_tracker_account_data
//                 .serialize(&mut &mut args.percent_tracker_pda.data.borrow_mut()[..])
//                 .map_err(|_| ProgramError::InvalidAccountData)?;
//         }

//         let mut user_claim_tracker_account_data =
//             try_from_slice_unchecked::<Tracker>(&args.user_claim_tracker_pda.data.borrow())
//                 .map_err(|_| ProgramError::InvalidAccountData)?;

//         if user_claim_tracker_account_data.increment < percent_tracker_account_data.increment {
//             let user_raydium_pool_lp_ass_token_account_amount =
//                 TokenAccount::unpack(&args.user_simple_pool_lp_ass_token_account.data.borrow())
//                     .unwrap()
//                     .amount;

//             let raydium_pool_lp_token_mint_supply =
//                 Mint::unpack(&args.simple_pool_lp_token_mint_account.data.borrow())
//                     .map_err(|_| ProgramError::InvalidAccountData)?
//                     .supply;

//             let user_raydium_lp_ratio = user_raydium_pool_lp_ass_token_account_amount as f64
//                 / raydium_pool_lp_token_mint_supply as f64;

//             let user_claim_percent =
//                 percent_tracker_account_data.increment - user_claim_tracker_account_data.increment;

//             let user_share =
//                 total_drainable_simple * user_claim_percent as f64 / 100.0 * user_raydium_lp_ratio;

//             msg!(
//                 "user claim incr. start: {}",
//                 user_claim_tracker_account_data.increment
//             );
//             msg!(
//                 "percent tracker: {}",
//                 percent_tracker_account_data.increment
//             );
//             msg!("user's claim percent: {}%", user_claim_percent);
//             msg!(
//                 "max amount of simple claimable by user: {} simple",
//                 user_claim_percent as f64 / 100.0 * total_drainable_simple
//                     / LAMPORTS_PER_SOL as f64
//             );
//             msg!("user's LP ratio: {}", user_raydium_lp_ratio);
//             msg!(
//                 "user's actual share (not divided by 1B): {} simple",
//                 user_share as u64
//             );

//             transfer_simple(
//                 &[
//                     args.simple_token_mint_account.clone(),
//                     args.program_simple_token_ass_account.clone(),
//                     args.user_simple_token_ass_account.clone(),
//                     args.transfer_signer_pda.clone(),
//                     args.user.clone(),
//                     args.user.clone(),
//                     args.system_program.clone(),
//                     args.token_program.clone(),
//                     args.associated_token_program.clone(),
//                 ],
//                 user_share as u64,
//                 program_id,
//             )?;

//             let user_simple_token_ass_account_amount =
//                 TokenAccount::unpack(&args.user_simple_token_ass_account.data.borrow())
//                     .unwrap()
//                     .amount;

//             msg!(
//                 "user simple wallet balance: {} simple",
//                 user_simple_token_ass_account_amount
//             );

//             user_claim_tracker_account_data.increment = percent_tracker_account_data.increment;
//             user_claim_tracker_account_data
//                 .serialize(&mut &mut args.user_claim_tracker_pda.data.borrow_mut()[..])
//                 .map_err(|_| ProgramError::InvalidAccountData)?;

//             msg!(
//                 "user claim incr. end {}",
//                 user_claim_tracker_account_data.increment
//             )
//         }
//     }

//     Ok(())
// }
