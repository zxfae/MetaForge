use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub header: String,
    //Hash SHA-256 (or other)
    pub prev_hash: [u8; 32],
    //Chronological ordering
    pub time_stamp: DateTime<Utc>,
    //pub nonce: u64,
}

impl Block {
    //Constructs new instance
    pub fn new(header: String, prev_hash: [u8; 32]) -> Self {
        Block {
            header,
            prev_hash,
            time_stamp: Utc::now(),
            //nonce: 0,
        }
    }

    pub fn formatted_timestamp_with_month(&self) -> String {
        self.time_stamp.format("%b %d, %Y, %H:%M:%S").to_string()
    }

    //@Dev : Generated form the timestamp, given data and the last hash
    pub fn calculate_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        Digest::update(&mut hasher, &self.header.as_bytes());
        Digest::update(&mut hasher, &self.prev_hash);
        Digest::update(&mut hasher, &self.time_stamp.to_rfc3339().as_bytes());

        let result = hasher.finalize();
        let mut hash = [0; 32];
        hash.copy_from_slice(&result);
        hash
    }

    pub fn hash_to_hex(&self) -> String {
        hex::encode(self.calculate_hash())
    }
}

#[cfg(test)]
mod test;
