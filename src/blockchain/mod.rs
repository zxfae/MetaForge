use crate::blocks::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new("Genesis Block".to_string(), [0; 32]);
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add(&mut self, data: String) -> &Block {
        //get prev block && hash
        let prev_block = self.blocks.last().unwrap();
        let prev_hash = prev_block.calculate_hash();

        let new_block = Block::new(data, prev_hash);

        self.blocks.push(new_block);
        self.blocks.last().unwrap()
    }

    //Verify [i] && [i-1] matching block
    pub fn verify(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let prev = &self.blocks[i - 1];

            if current.prev_hash != prev.calculate_hash() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test;
