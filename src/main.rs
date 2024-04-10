use solana_client::rpc_client::RpcClient;
use rust_client::{check_balance, create_ata, create_keypair, request_air_drop, transfer_funds, transfer_spl_token};
use solana_sdk::{address_lookup_table::instruction, program_pack::Pack, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token::state::Mint;
use serde::{Deserialize};
use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;


#[derive(Deserialize)]
struct RPC_Node {
    rpc_node: String,
}


const URL: &str = "https://api.devnet.solana.com";
// const URL: &str = "http://localhost:8899";


fn main() {
    let file = File::open("setting.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let rpc_node_config: RPC_Node = from_reader(reader).expect("Failed to parse JSON");

    println!("rpc_node: {}", rpc_node_config.rpc_node);

    let rpc_client = RpcClient::new(URL);

    let sender = create_keypair();
    let receiver = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    if let Ok(airdrop_signature) = request_air_drop(&rpc_client, &sender.pubkey(), 1.0) {
        println!("Airdrop finished! Signature: {:?}",  airdrop_signature);

        if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
            println!("Sender balance: {:?}", balance);
        }

        let transfer_amount = 0.5;

        match transfer_funds(&rpc_client, &sender, &receiver.pubkey(), transfer_amount) {
            Ok(sig) => { 
                println!("Transfer of {:?} finished. Signature: {:?}", transfer_amount, sig);
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }

        let mint_account = create_keypair();
        let token_program_id = spl_token::id();
        let lamports = 5000000;
        let token_mint = solana_program::system_instruction::create_account(&sender.pubkey(), &mint_account.pubkey(), lamports, Mint::LEN as u64, &token_program_id);

        let token_mint_init = spl_token::instruction::initialize_mint(&token_program_id, &mint_account.pubkey(), &sender.pubkey(), None, 9).unwrap();

        let block_hash = rpc_client.get_latest_blockhash().unwrap();
        let token_transaction = Transaction::new_signed_with_payer(&[token_mint, token_mint_init], Some(&sender.pubkey()), &[&sender, &mint_account], block_hash);
        // token_transaction.sign(&[sender], block_hash);
        match rpc_client.send_and_confirm_transaction(&token_transaction) {
            Ok(sig) => {
                println!("created token account: {:?}", sig);
            },
            Err(err) => println!("Error: {:?}", err),
        }

        println!("token mint:{}", token_program_id);


        match create_ata(&rpc_client, &sender.pubkey(), &sender, &mint_account.pubkey(), &token_program_id) {
            Ok(sig) => { 
                println!("Transfer of {:?} finished. Signature: {:?}", transfer_amount, sig);
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }

        match create_ata(&rpc_client, &receiver.pubkey(), &sender, &mint_account.pubkey(), &token_program_id) {
            Ok(sig) => { 
                println!("Transfer of {:?} finished. Signature: {:?}", transfer_amount, sig);
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }

        let sender_ata = spl_associated_token_account::get_associated_token_address(&sender.pubkey(), &mint_account.pubkey());
        let receiver_ata = spl_associated_token_account::get_associated_token_address(&receiver.pubkey(), &mint_account.pubkey());
        

        match transfer_spl_token(&rpc_client, &token_program_id, &sender, &sender_ata, &receiver_ata, 0) {
            Ok(sig) => { 
                println!("Transfer of {:?} finished. Signature: {:?}", transfer_amount, sig);
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }


    } else {
        println!("Airdrop failed");
    }


}
