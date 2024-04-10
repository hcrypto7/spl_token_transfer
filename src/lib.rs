use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    signature::{Keypair, Signature}, signer::{keypair, Signer}, stake::instruction, system_transaction, sysvar::recent_blockhashes, transaction::Transaction
};
use std::{error::Error, io::Sink};

const LAMPORTS_PER_SOL: f64 = 1000000000.0;

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)? as f64 / LAMPORTS_PER_SOL)
}

pub fn request_air_drop(
    rpc_client: &RpcClient,
    pub_key: &Pubkey,
    amount_sol: f64,
) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn transfer_funds(
    rpc_client: &RpcClient,
    sender_keypair: &Keypair,
    receiver_pub_key: &Pubkey,
    amount_sol: f64,
) -> core::result::Result<Signature, Box<dyn Error>> {
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL) as u64;

    Ok(
        rpc_client.send_and_confirm_transaction(&system_transaction::transfer(
            &sender_keypair,
            &receiver_pub_key,
            amount_lamports,
            rpc_client.get_latest_blockhash()?,
        ))?,
    )
}

pub fn transfer_spl_token(
    rpc_client: &RpcClient,
    token_program_id: &Pubkey,
    main_signer: &Keypair,
    sender_ata: &Pubkey,
    receiver_ata: &Pubkey,
    amount_token: u64,
) -> core::result::Result<Signature, Box<dyn Error>> {
    let instruction = spl_token::instruction::transfer(
        &token_program_id,
        &sender_ata,
        &receiver_ata,
        &main_signer.pubkey(),
        &[],
        amount_token,
    )?;
    let block_hash = rpc_client.get_latest_blockhash()?;
    let transaction =
        Transaction::new_signed_with_payer(&[instruction], Some(&main_signer.pubkey()), &[&main_signer], block_hash);
    Ok(rpc_client.send_and_confirm_transaction(&transaction)?)
}

pub fn create_ata(
    rpc_client: &RpcClient,
    receiver_key: &Pubkey,
    signer_keypair: &Keypair,
    token_address: &Pubkey,
    token_program_id: &Pubkey,
) -> core::result::Result<Signature, Box<dyn Error>> {
    let instruction = spl_associated_token_account::instruction::create_associated_token_account(
        &signer_keypair.pubkey(),
        &receiver_key,
        &token_address,
        &token_program_id,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer_keypair.pubkey()),
        &[&signer_keypair],
        rpc_client.get_latest_blockhash()?,
    );

    Ok(rpc_client.send_and_confirm_transaction(&transaction)?)
}
