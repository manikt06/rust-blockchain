use chrono::Utc;

use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
        };

        blockchain.create_genesis_block();

        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis = Block::new(
            0,
            Utc::now().to_string(),
            "Genesis Block".to_string(),
            "0".to_string(),
        );

        self.chain.push(genesis);
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();

        let block = Block::new(
            previous_block.index + 1,
            Utc::now().to_string(),
            data,
            previous_block.hash.clone(),
        );

        self.chain.push(block);
    }

    pub fn display_chain(&self) {
        for block in &self.chain {
            println!("------------------------");
            println!("Index         : {}", block.index);
            println!("Timestamp     : {}", block.timestamp);
            println!("Data          : {}", block.data);
            println!("Previous Hash : {}", block.previous_hash);
            println!("Hash          : {}", block.hash);
        }
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let recalculated_hash = Block::calculate_hash(
                current.index,
                &current.timestamp,
                &current.data,
                &current.previous_hash,
            );

            if current.hash != recalculated_hash {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }

        true
    }
}