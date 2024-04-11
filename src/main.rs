use rust_client::{
    check_balance, create_ata, create_keypair, transfer_funds, transfer_spl_token,
};
use serde::Deserialize;
use serde_json::from_reader;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey, signature::Keypair, signer::Signer
};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Deserialize)]
struct ProgramConfig {
    rpc_node: String,
    private_key: String,
}

fn main() {
    //--------------------------Set RPC Node from setting.json-----------------------------//
    let file = File::open("setting.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let program_config: ProgramConfig = from_reader(reader).expect("Failed to parse JSON");

    println!("Rpc_node: {}", &program_config.rpc_node);
    let rpc_client = RpcClient::new(&program_config.rpc_node);

    // Create a keypair from the private key string
    let sender = Keypair::from_base58_string(&program_config.private_key);

    println!("Private Key: {:?}", sender.to_base58_string());
    println!("Public Key: {:?}", sender.pubkey().to_string());

    let receiver = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    // if let Ok(airdrop_signature) = request_air_drop(&rpc_client, &sender.pubkey(), 1.0) {
    //     println!("Airdrop finished! Signature: {:?}", airdrop_signature);

    if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
        println!("Sender balance: {:?} Sol", balance);
    }

    //-----------------------------Load data from data.csv---------------------------------//
    let list_file = File::open("data.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(list_file);

    for result in rdr.records() {
        println!(">>>>>--------------------------------------------------------------------------->>>>>");
        let record = result.unwrap();
        // println!("{:?}", &record);

        let transfer_amount = record[2].trim().parse::<f64>().unwrap();
        let receiver = Pubkey::from_str(&record[1].trim()).expect("Failed to create Pubkey from string");
        let token_address = Pubkey::from_str(&record[0].trim()).expect("Failed to create Token Address from string");

        match transfer_funds(&rpc_client, &sender, &receiver, transfer_amount) {
            Ok(sig) => {
                println!(
                    "Transfer of {:?} finished. Signature: {:?}",
                    transfer_amount, sig
                );
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc_client, &receiver) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            }
            Err(err) => println!("Error: {:?}", err),
        }

        // let mint_account = create_keypair();
        let token_program_id = spl_token::id();
        // let lamports = 5000000;
        // let token_mint = solana_program::system_instruction::create_account(
        //     &sender.pubkey(),
        //     &mint_account.pubkey(),
        //     lamports,
        //     Mint::LEN as u64,
        //     &token_program_id,
        // );

        // let token_mint_init = spl_token::instruction::initialize_mint(
        //     &token_program_id,
        //     &mint_account.pubkey(),
        //     &sender.pubkey(),
        //     None,
        //     9,
        // )
        // .unwrap();

        // let block_hash = rpc_client.get_latest_blockhash().unwrap();
        // let token_transaction = Transaction::new_signed_with_payer(
        //     &[token_mint, token_mint_init],
        //     Some(&sender.pubkey()),
        //     &[&sender, &mint_account],
        //     block_hash,
        // );
        // token_transaction.sign(&[sender], block_hash);
        // match rpc_client.send_and_confirm_transaction(&token_transaction) {
        //     Ok(sig) => {
        //         println!("created token account: {:?}", sig);
        //     }
        //     Err(err) => println!("Error: {:?}", err),
        // }

        // println!("token mint:{}", token_program_id);

        // let sender_ata_temp = spl_associated_token_account::get_associated_token_address(
        //     &sender.pubkey(),
        //     &token_address,
        // );
        // let receiver_ata_temp = spl_associated_token_account::get_associated_token_address(
        //     &receiver,
        //     &token_address,
        // );

        // println!("sender_ata: {:?}, receiver_ata: {:?}", sender_ata_temp, receiver_ata_temp);


        match create_ata(
            &rpc_client,
            &sender.pubkey(),
            &sender,
            &token_address,
            &token_program_id,
        ) {
            Ok(sig) => {
                // println!(
                //     "Transfer of {:?} finished. Signature: {:?}",
                //     transfer_amount, sig
                // );
                // if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                //     println!("Sender balance after transfer: {:?}", balance);
                // }
                // if let Ok(balance) = check_balance(&rpc_client, &receiver) {
                //     println!("Receiver balance after transfer: {:?}", balance);
                // }
            }
            Err(err) =>{} //println!("Error: {:?}", err),
        }

        match create_ata(
            &rpc_client,
            &receiver,
            &sender,
            &token_address,
            &token_program_id,
        ) {
            Ok(sig) => {
                // println!("account created! signature: {:?}", sig);
                // println!(
                //     "Transfer of {:?} finished. Signature: {:?}",
                //     transfer_amount, sig
                // );
                // if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                //     println!("Sender balance after transfer: {:?}", balance);
                // }
                // if let Ok(balance) = check_balance(&rpc_client, &receiver) {
                //     println!("Receiver balance after transfer: {:?}", balance);
                // }
            }
            Err(err) => {}//println!("account exist"), //println!("Error: {:?}", err),
        }

        let sender_ata = spl_associated_token_account::get_associated_token_address(
            &sender.pubkey(),
            &token_address,
        );
        let receiver_ata = spl_associated_token_account::get_associated_token_address(
            &receiver,
            &token_address,
        );

        match transfer_spl_token(
            &rpc_client,
            &token_program_id,
            &sender,
            &sender_ata,
            &receiver_ata,
            0,
        ) {
            Ok(sig) => {
                println!("{:?} => {:?} 0 token transfered : sig = {:?}", sender.pubkey(), receiver, sig);
                // println!("0 token has transfered: {:?}", sig);
                // println!(
                //     "Transfer of {:?} finished. Signature: {:?}",
                //     transfer_amount, sig
                // );
                // if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                //     println!("Sender balance after transfer: {:?}", balance);
                // }
                // if let Ok(balance) = check_balance(&rpc_client, &receiver) {
                //     println!("Receiver balance after transfer: {:?}", balance);
                // }
            }
            Err(err) => println!("error occured"),//println!("Error: {:?}", err),
        }
    }


    println!("Press any key to end the program...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // } else {
    //     println!("Airdrop failed");
    // }
}
