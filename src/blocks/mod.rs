use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub time_stamp: DateTime<Utc>,
    pub prev_hash: [u8; 32],
    pub hash: [u8; 32],
    pub data: Vec<String>,
}

impl Block {
    pub fn new(data: Vec<String>, prev_hash: [u8; 32]) -> Self {
        let time_stamp = Utc::now();
        let mut block = Block {
            time_stamp,
            prev_hash,
            hash: [0; 32],
            data,
        };

        block.hash = block.calculate_hash();
        block
    }

    //@Notice prev_hash for genesis_block == 0
    pub fn genesis_block() -> Self {
        Block::new(vec!["Genesis_Block !".to_string()], [0; 32])
    }

    pub fn formatted_timestamp_with_month(&self) -> String {
        self.time_stamp.format("%b %d, %Y, %H:%M:%S").to_string()
    }

    //@Dev : Generated form the timestamp, given data and the last hash
    pub fn calculate_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        Digest::update(&mut hasher, &self.time_stamp.to_rfc3339().as_bytes());
        Digest::update(&mut hasher, &self.prev_hash);

        for values in &self.data {
            hasher.update(values.as_bytes());
        }

        let result = hasher.finalize();
        let mut hash = [0; 32];
        hash.copy_from_slice(&result);
        hash
    }
    //@Quest : Need refacto hash && last hash encode
    //@Dev : Formatting hash_to_hex
    pub fn hash_to_hex(&self) -> String {
        //high performance cost
        //hex::encode(self.calculate_hash())
        hex::encode(self.hash)
    }

    pub fn last_hash_to_hex(&self) -> String {
        hex::encode(self.prev_hash)
    }
}

#[cfg(test)]
mod test;
