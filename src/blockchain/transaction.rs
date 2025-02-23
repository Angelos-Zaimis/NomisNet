use std::ptr::read;
use secp256k1::{All, Message, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use hex;
use sha2::{Digest, Sha256};

const BASE_FEE: f64 = 0.1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub fee: f64,
}

#[derive(Debug, Clone)]
pub struct SignedTransaction {
    transaction: Transaction,
    signature: Vec<u8>
}

impl Transaction {

    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        Self {
            sender,
            receiver,
            amount,
            fee: BASE_FEE
        }
    }

    pub fn sing_transaction(transaction: &Transaction, private_key: &str) -> Result<SignedTransaction, String> {
        let secp: Secp256k1<All> = Secp256k1::new();

        let private_key: SecretKey = Transaction::get_private_key(&private_key)?;

        let transaction_hash = Self::hash_transaction(transaction);

        let message = Self::get_message(&transaction_hash).expect("Failed to get the message");

        let signature = secp.sign_ecdsa(&message, &private_key);

        Ok(SignedTransaction {
            transaction: transaction.clone(),
            signature: signature.serialize_compact().to_vec()
        })
    }

    fn get_message(transaction_hash: &[u8]) -> Result<Message, String>{
        match Message::from_slice(transaction_hash) {
            Ok(message) => Ok(message),
            Err(_) => return Err("Invalid message length".to_string())
        }
    }

    // Decode hex string into bytes and convert them into a SecretKey
    fn get_private_key(private_key: &str) -> Result<SecretKey, String> {
        match SecretKey::from_slice(&hex::decode(private_key).expect("Failed to decode private key")) {
            Ok(secret_key) => {
                Ok(secret_key)
            }
            Err(_) => return Err("Invalid private key".to_string())
        }
    }

    fn hash_transaction(tx: &Transaction) -> [u8; 32] {
        let serialized_tx: Vec<u8> = serde_json::to_vec(tx).expect("Failed to serialize transaction");
        let hash = Sha256::digest(&serialized_tx);
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash);
        result
    }
}

