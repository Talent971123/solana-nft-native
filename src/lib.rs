use borsh::{BorshDeserialize, BorshSerialize};

use mpl_token_metadata::instruction as mpl_instruction;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar, //new
};
use spl_token::{instruction as token_instruction, state::Mint}; //new
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let token_metadata = CreateTokenArgs::try_from_slice(instruction_data)?;
    create_nft(accounts, token_metadata)?;

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CreateTokenArgs {
    pub nft_title: String,
    pub nft_symbol: String,
    pub nft_uri: String,
}

fn create_nft(accounts: &[AccountInfo], create_token_data: CreateTokenArgs) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;

    msg!("creating mint account....");
    msg!("Mint: {}", mint_authority.key);
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            &token_program.key,
        ),
        &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Creating mint account....");
    msg!("Mint: {}", mint_account.key);

    invoke(
        &token_instruction::initialize_mint(
            &token_program.key,
            &mint_account.key,
            &mint_authority.key,
            Some(&mint_authority.key),
            0,
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            rent.clone(),
        ],
    )?;

    msg!("Creating metadata account....");
    msg!("Metadata account address: {}", metadata_account.key);

    invoke(
        &mpl_instruction::create_metadata_accounts_v3(
            *token_program.key,
            *metadata_account.key,
            *mint_account.key,
            *mint_authority.key,
            *payer.key,
            *mint_authority.key,
            create_token_data.nft_title,
            create_token_data.nft_symbol,
            create_token_data.nft_uri,
            None,
            0,
            true,  //is the payer the update authority
            false, //can we update the token metadata
            None,  //collection the nft belongs to. struct of pubkey and
            None,  //uses
            None,  //collection details
        ),
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            token_metadata_program.clone(),
            rent.clone(),
        ],
    )?;
    msg!("Token mint created successfully.");

    Ok(())
}
