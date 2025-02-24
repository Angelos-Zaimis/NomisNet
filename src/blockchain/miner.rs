use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::mempool::Mempool;
use crate::blockchain::transaction::Transaction;

const MIN_TRANSACTIONS_FOR_BLOCK: usize = 2;
const ADJUSTMENT_BLOCK_COUNT: usize = 5;

pub struct Miner<'a> {
    blockchain: &'a Blockchain<'a>,
}

const DIFFICULTY: u32 = 4;

impl<'a> Miner<'a> {


    pub fn new(blockchain: &'a Blockchain<'a>) -> Self {
        Self {
            blockchain
        }
    }

    pub fn mine_block(&self, mempool: &Mempool) {
        let mut transactions: Vec<Transaction> = mempool.get_sorted_transactions();

        if transactions.len() < MIN_TRANSACTIONS_FOR_BLOCK {
            println!("Not enough transactions in the mempool to mine a block. Waiting...");
            return;
        }

        let total_fees: f64 = Self::get_total_fees(&transactions);
        let selected_transactions = Self::select_transactions_for_block(&mut transactions);
        let data: String = format!("{:?}", selected_transactions);
        let target_prefix: String = "0".repeat(DIFFICULTY as usize);
        let miner_address: String = "Change it".to_string();
        let last_block = self.blockchain.get_last_block().expect("Failed to get last block");

        self.start_mining(target_prefix, &data, &miner_address, total_fees, &last_block);
    }

    fn start_mining(&self, target_prefix: String, data: &str, miner_address: &str, total_fees: f64, last_block: &Block) {
        let mut new_block: Block = Block::new(last_block.index + 1, last_block.hash.clone(), data.to_string(), DIFFICULTY);
        let start_time = std::time::Instant::now();

        while !new_block.hash.starts_with(&target_prefix) {
            new_block.nonce += 1;
            new_block.hash = Block::calculate_hash(
                new_block.index,
                &new_block.timestamp,
                &new_block.previous_hash,
                &new_block.data,
                new_block.nonce,
                &DIFFICULTY,
            );
        }

        let mining_time = start_time.elapsed().as_secs();

        println!(
            "Block Mined! Nonce: {}, Hash: {}, Difficulty: {}, Time: {} sec",
            new_block.nonce, new_block.hash, &DIFFICULTY, mining_time
        );

        self.blockchain.add_new_block(&new_block);

        // TO-DO
        Self::reward_the_miner(&*"".to_string(), &total_fees)
    }


    //to-do
    fn adjust_difficulty(&self) {
    }
    //TO DO
    fn reward_the_miner(miner_address: &str, total_fees: &f64) {

    }
    fn select_transactions_for_block(transactions: &[Transaction]) -> Vec<Transaction> {
        let mut sorted_transactions = transactions.to_vec();
        sorted_transactions.sort_by(|a, b| b.fee.partial_cmp(&a.fee).unwrap());
        sorted_transactions
    }

    fn get_total_fees(transactions: &Vec<Transaction>) -> f64 {
        transactions.iter().map(|t|  t.fee).sum()
    }
}