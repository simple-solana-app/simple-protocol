// use {
//     solana_program::{
//         account_info::{next_account_info, AccountInfo},
//         entrypoint::ProgramResult,
//         msg,
//         program::{invoke, invoke_signed},
//         pubkey::Pubkey,
//     },
//     spl_associated_token_account::instruction as associated_token_account_instruction,
// };

// pub fn transfer_simple(
//     accounts: &[AccountInfo],
//     user_share: u64,
//     program_id: &Pubkey,
// ) -> ProgramResult {
//     let accounts_iter = &mut accounts.iter();

//     let mint_account = next_account_info(accounts_iter)?;
//     let from_associated_token_account = next_account_info(accounts_iter)?;
//     let to_associated_token_account = next_account_info(accounts_iter)?;
//     let owner = next_account_info(accounts_iter)?;
//     let recipient = next_account_info(accounts_iter)?;
//     let payer = next_account_info(accounts_iter)?;
//     let system_program = next_account_info(accounts_iter)?;
//     let token_program = next_account_info(accounts_iter)?;
//     let associated_token_program = next_account_info(accounts_iter)?;

//     if to_associated_token_account.lamports() == 0 {
//         msg!("Creating associated token account for recipient...");
//         invoke(
//             &associated_token_account_instruction::create_associated_token_account(
//                 payer.key,
//                 recipient.key,
//                 mint_account.key,
//                 token_program.key,
//             ),
//             &[
//                 mint_account.clone(),
//                 to_associated_token_account.clone(),
//                 recipient.clone(),
//                 payer.clone(),
//                 system_program.clone(),
//                 token_program.clone(),
//                 associated_token_program.clone(),
//             ],
//         )?;
//     } else {
//         msg!("Associated token account exists.");
//     }
//     msg!(
//         "Recipient Associated Token Address: {}",
//         to_associated_token_account.key
//     );

//     msg!("Transferring {} tokens...", user_share);
//     msg!("Mint: {}", mint_account.key);
//     msg!("Owner Token Address: {}", from_associated_token_account.key);
//     msg!(
//         "Recipient Token Address: {}",
//         to_associated_token_account.key
//     );

//     let seed = b"transfer_signer_pda";
//     let (_transfer_signer_address, bump_seed) = Pubkey::find_program_address(&[seed], program_id);
//     invoke_signed(
//         &token_instruction::transfer(
//             token_program.key,
//             from_associated_token_account.key,
//             to_associated_token_account.key,
//             owner.key,
//             &[owner.key, recipient.key],
//             user_share,
//         )?,
//         &[
//             mint_account.clone(),
//             from_associated_token_account.clone(),
//             to_associated_token_account.clone(),
//             owner.clone(),
//             recipient.clone(),
//             token_program.clone(),
//         ],
//         &[&[seed, &[bump_seed]]], // Signer seeds (for PDA authority)
//     )?;

//     Ok(())
// }
