use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use secp256k1::PublicKey;
use sha2::{Digest, Sha256};
use crate::blockchain::transaction::Transaction;

const MAX_MEMPOOL_SIZE: usize = 10_000;

#[derive(Debug)]
pub struct Mempool {
    transactions: Arc<RwLock<HashMap<String, Transaction>>>,
}


impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction, sender_public_key: &PublicKey) -> Result<(), String> {
        let mut pool = self.transactions.write().expect("Failed acquire write lock");

        if pool.len() >= MAX_MEMPOOL_SIZE {
            return Err("Mempool is full".to_string());
        }

        let tx_hash = Mempool::hash_transaction(&transaction);
        pool.insert(tx_hash, transaction);
        Ok(())
    }


    pub fn get_sorted_transactions(&self) -> Vec<Transaction> {
        let pool = self.transactions.read().expect("Failed to acquire read lock");
        let mut transactions: Vec<Transaction> = pool.values().cloned().collect();
        transactions.sort_by(|a, b| b.fee.partial_cmp(&a.fee).unwrap());
        transactions
    }

    // TO-DO: remove transaction

    //TO-DO: get transaction
    pub fn contains_transaction(&self, transaction_hash: &str) -> bool {
        let pool = self.transactions.read().expect("Failed to acquire read lock");
        pool.contains_key(transaction_hash)
    }

    fn hash_transaction(transaction: &Transaction) -> String {
        let serialized_transaction = serde_json::to_vec(transaction).expect("Failed to serialize transaction");
        let hash = Sha256::digest(&serialized_transaction);
        hex::encode(hash)
    }
}