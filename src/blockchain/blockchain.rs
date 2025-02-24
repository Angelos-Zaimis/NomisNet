use crate::blockchain::block::Block;
use crate::db::blockchaindb::BlockChainDB;

pub struct Blockchain<'a> {
    db: &'a BlockChainDB,
}

impl<'a> Blockchain<'a> {
    pub fn new(db: &'a BlockChainDB) -> Self {
        Blockchain { db }
    }

    pub fn add_new_block(&self, new_block: &Block) {
        self.db.save_new_block(new_block)
    }
    pub fn get_block_chain(&self) -> Vec<Block> {
        self.db.get_all_blocks()
    }
    pub fn get_last_block(&self) -> Option<Block> {
        self.db.get_last_block()
    }

    pub fn get_block(&self, block_number: u64) -> Option<Block> {
        self.db.get_block(block_number)
    }
}
