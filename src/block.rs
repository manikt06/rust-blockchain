use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: String,
        data: String,
        previous_hash: String,
    ) -> Self {
        let hash = Self::calculate_hash(
            index,
            &timestamp,
            &data,
            &previous_hash,
        );

        Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(
        index: u32,
        timestamp: &str,
        data: &str,
        previous_hash: &str,
    ) -> String {
        let input = format!(
            "{}{}{}{}",
            index,
            timestamp,
            data,
            previous_hash
        );

        let mut hasher = Sha256::new();
        hasher.update(input);

        format!("{:x}", hasher.finalize())
    }
}