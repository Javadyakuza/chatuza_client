extern crate crypto;
use crate::structs::TransferResponse;
use crate::structs::{MnemonicInput, NewAccountOutput};
use bip39::Mnemonic;
use rand::Rng;
use reqwest::{Client, Request};
use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
    signer::{keypair::Keypair, EncodableKey, SeedDerivable, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use std::path::Path;

// Notice that because these functions are client side function we return the direct displayable messages to the user
// EH
pub fn add_existing_account<'a>(
    mnemonic_in: &MnemonicInput,
) -> Result<NewAccountOutput<'a>, String> {
    // decrypting the
    let mi: String = mnemonic_in.to_string();
    let kp: Keypair;
    match Keypair::from_seed(mi.as_bytes()) {
        Ok(_kp) => kp = _kp,
        Err(e) => {
            return Err(format!(
                "could't build the key pair from the seed due to \n {:?}",
                e
            ))
        }
    }
    let pk: Pubkey;
    match kp.try_pubkey() {
        Ok(_pk) => pk = _pk,
        Err(e) => {
            return Err(format!(
                "could't build the addr from key pair derived from the seed due to \n {:?}",
                e
            ))
        }
    }
    Ok(NewAccountOutput {
        mnemonic: None,
        pub_key: pk.to_string(),
        keypair: kp.to_base58_string(),
    })
}

// Generate new account
pub fn gen_new_account<'a>(regenerate: bool) -> Result<NewAccountOutput<'a>, String> {
    if Path::new("sec.json").is_file() && !regenerate {
        return Err(format!(
            "you already have a account !
            if you want a new account delete the previous one
            ❌ DISCLAIMER : deleting an account can result to permanent loss of the account owned assets ❌"
        ));
    }
    // generating new mnemonic and its hash
    let backup_unattached: Mnemonic;
    match get_mnemonic() {
        Ok(mne) => backup_unattached = mne,
        Err(e) => return Err(format!("{}", e)),
    } // handled in the called function
      // generating new key pair
    let kp: Keypair;
    match Keypair::from_seed(backup_unattached.to_string().as_bytes()) {
        Ok(_kp) => kp = _kp,
        Err(e) => {
            return Err(format!(
                "could't build the key pair from the seed due to \n {:?}",
                e
            ))
        }
    }
    let mut mi_arr: [&'a str; 12] = Default::default();
    let mi = backup_unattached.word_iter().collect::<Vec<&'_ str>>();
    mi_arr.copy_from_slice(&mi);

    let pk: Pubkey;
    match kp.try_pubkey() {
        Ok(_pk) => pk = _pk,
        Err(e) => {
            return Err(format!(
                "could't build the addr from key pair derived from the seed due to \n {:?}",
                e
            ))
        }
    }
    // saving keypair
    if let Err(_) = kp.write_to_file("sec.json") {
        return Err(format!("error while saving the key pair"));
    }

    Ok(NewAccountOutput {
        mnemonic: Some(MnemonicInput { words: mi_arr }),
        pub_key: pk.to_string(),
        keypair: kp.to_base58_string(),
    })
}
// EH
fn get_mnemonic() -> Result<Mnemonic, String> {
    // GEN RAW
    let mut entropy: [u8; 16] = [0; 16];
    rand::thread_rng().fill(&mut entropy);
    match Mnemonic::from_entropy(&entropy) {
        Ok(m) => Ok(m),
        Err(e) => return Err(format!("Error while generating the Mnemonic \n {:?}", e)),
    }
}

//EH
pub fn transfer_spl(
    program_id: &Pubkey, // supporting various programs
    mint_pubkey: &Pubkey,
    recipient_wallet_pubkey: &Pubkey,
    amount: u64, // without decimals
    decimals: u8,
) -> Result<TransferResponse, String> {
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
    let lbh: Hash;
    match rpc.get_latest_blockhash() {
        Ok(hash) => lbh = hash,
        Err(e) => {
            return Err(format!(
                "Error while etching the latest block hash \n {:?}",
                e
            ))
        }
    }
    let sender_keypair: Keypair;
    match Keypair::read_from_file("sec.json") {
        Ok(_kp) => sender_keypair = _kp,
        Err(e) => return Err(format!("Error while reading the signer info \n {:?}", e)),
    }
    let sender_pubkey = sender_keypair.try_pubkey().unwrap(); // panic impossible

    // getting the token account of two side of the deal
    let sender_token_acc: Pubkey =
        get_associated_token_address_with_program_id(&sender_pubkey, &mint_pubkey, &program_id);
    println!("{}", sender_token_acc.to_string());
    let recipient_token_acc: Pubkey = get_associated_token_address_with_program_id(
        &recipient_wallet_pubkey,
        &mint_pubkey,
        &program_id,
    );

    if rpc.get_account(&sender_token_acc).is_err() {
        return Err(format!(
            "user {:?} doesn't have any account nor any balance of the {} token",
            sender_pubkey, mint_pubkey
        ));
    }

    if let Err(_) = rpc.get_account(&recipient_wallet_pubkey) {
        // wallet is unfunded, funding
        if let Err(e) = fund_account(recipient_wallet_pubkey.to_string()) {
            return Err(format!(
                "failed to fund the recipient wallet due to \n {}",
                e
            ));
        }
    }; // reentrance impossible, second time account will be considered as active.

    let mut create_recipient_acc_sigs: Vec<Option<String>> = vec![None, None];

    if let Err(_) = rpc.get_account(&recipient_token_acc) {
        // account was not created, creating
        let tmp_signatures: Vec<String>;
        match create_token_account(
            recipient_wallet_pubkey.to_string(),
            mint_pubkey.to_string(),
            program_id.to_string(),
            lbh.to_string(),
        ) {
            Ok(tmp) => tmp_signatures = tmp,
            Err(e) => {
                return Err(format!(
                    "failed to create the associated token account due to \n {}",
                    e
                ))
            }
        }
        create_recipient_acc_sigs = vec![
            Some(tmp_signatures[0].to_owned()),
            Some(tmp_signatures[1].to_owned()),
        ];
    };

    let transfer_signature: Signature;
    // sending the transaction
    match spl_token::instruction::transfer_checked(
        &program_id,
        &sender_token_acc,
        &mint_pubkey,
        &recipient_token_acc,
        &sender_pubkey,
        &[&sender_pubkey],
        amount * (10 as u64).pow(9),
        decimals,
    ) {
        Ok(ix) => match rpc.send_and_confirm_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&sender_pubkey),
            &[&sender_keypair],
            lbh,
        )) {
            Ok(sig) => transfer_signature = sig,
            Err(e) => return Err(format!("transaction failed due to \n {}", e)),
        },
        Err(e) => return Err(format!("failed to create the instruction due to \n {}", e)),
    };

    Ok(TransferResponse {
        create_account_sig: create_recipient_acc_sigs[0].to_owned(),
        funding_account_sig: create_recipient_acc_sigs[1].to_owned(),
        transfer_sig: transfer_signature.to_string(),
    })
}

// EH
#[tokio::main]
pub async fn create_token_account(
    wallet_address: String,
    token_mint_address: String,
    token_program_id: String,
    lbh: String,
) -> Result<Vec<String>, String> {
    let client = Client::new();

    let form_data = [
        ("wallet_address", wallet_address),
        ("token_mint_address", token_mint_address),
        ("token_program_id", token_program_id),
        ("lbh", lbh),
    ];

    let encoded_form_data = form_data
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&");

    let request = client
        .post("http://localhost:8000/api/create-token-account")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded_form_data);

    let built_request: Request;
    match request.build() {
        Ok(req) => built_request = req,
        Err(e) => {
            return Err(format!(
                "couldn't build the create account request due to {}",
                e
            ))
        }
    }

    let result: reqwest::Response;
    match client.execute(built_request).await {
        Ok(res) => result = res,
        Err(e) => return Err(format!("sending request failed due to \n {}", e)),
    }

    if result.status() != 200 {
        return Err(format!("server returned {}", result.status()));
    }

    let response_text: String;
    match result.text().await {
        Ok(text) => response_text = text,
        Err(e) => return Err(format!("couldn't fetch the response text due to \n {}", e)),
    }
    let response_val: Value;
    match serde_json::from_str(response_text.as_str()) {
        Ok(val) => response_val = val,
        Err(e) => return Err(format!("failed to parse the response due to {}", e)),
    }

    let signatures: Vec<String>;
    match response_val["Ok"]["signatures"].as_array() {
        Some(sigs) => signatures = sigs.iter().map(|signature| signature.to_string()).collect(),
        None => {
            return Err(format!(
                "failed to read the signatures ! \n server returned {}",
                response_val
            ))
        }
    }

    Ok(signatures)
}

#[tokio::main]
pub async fn fund_account(wallet_address: String) -> Result<String, String> {
    let client = Client::new();

    let form_data = [("wallet_address", wallet_address)];

    let encoded_form_data = form_data
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&");

    let request = client
        .post("http://localhost:8000/api/fund-wallet")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded_form_data);

    let built_request: Request;
    match request.build() {
        Ok(req) => built_request = req,
        Err(e) => {
            return Err(format!(
                "couldn't build the fund wallet request due to {}",
                e
            ))
        }
    }

    let result: reqwest::Response;
    match client.execute(built_request).await {
        Ok(res) => result = res,
        Err(e) => return Err(format!("sending request failed due to \n {}", e)),
    }

    if result.status() != 200 {
        return Err(format!("server returned {}", result.status()));
    }

    let response_text: String;
    match result.text().await {
        Ok(text) => response_text = text,
        Err(e) => return Err(format!("couldn't fetch the response text due to \n {}", e)),
    }
    let response_val: Value;
    match serde_json::from_str(response_text.as_str()) {
        Ok(val) => response_val = val,
        Err(e) => return Err(format!("failed to parse the response due to {}", e)),
    }

    // println!("{:?}", response_val);
    let signature: String;
    match response_val["Ok"].as_str() {
        Some(sig) => signature = sig.to_string(),
        None => {
            return Err(format!(
                "failed to read the signature ! \n server returned {}",
                response_val
            ))
        }
    }

    Ok(signature)
}

pub mod structs {
    use bip39::Mnemonic;
    use serde::Deserialize;

    #[derive(Debug, Default)]
    pub struct NewAccountOutput<'b> {
        pub mnemonic: Option<MnemonicInput<'b>>, // noted by the user
        pub pub_key: String, // to interact with the account (stored on the client sqlite)
        pub keypair: String, // to sign the transactions (stored on the client sqlite)
    }

    #[derive(Debug)]
    pub struct MnemonicOutput {
        pub hashed: String,
        pub raw: Mnemonic,
    }
    #[derive(Debug)]
    pub struct MnemonicInput<'a> {
        pub words: [&'a str; 12],
    }

    impl std::fmt::Display for MnemonicInput<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut temp_str: String = Default::default();
            for (index, word) in self.words.iter().enumerate() {
                if index == 0 {
                    temp_str.push_str(format!("{}", word).as_str());
                } else {
                    temp_str.push_str(format!(" {}", word).as_str());
                }
            }
            write!(f, "{}", temp_str)
        }
    }

    #[derive(Default)]
    pub struct CreateTokenAccount {
        pub wallet_address: String,
        pub token_mint_address: String,
        pub token_program_id: String,
        pub lbh: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct CreateTokenAccountResponse {
        pub signatures: Vec<String>,
    }

    #[derive(Debug)]
    pub struct TransferResponse {
        pub create_account_sig: Option<String>,
        pub funding_account_sig: Option<String>,
        pub transfer_sig: String,
    }
}
