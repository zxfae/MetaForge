use crate::blocks::Block;

//@Dev : Blockchain structure
#[derive(Debug, PartialEq, Eq)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::genesis_block()],
        }
    }

    pub fn add(&mut self, data: &Vec<String>) -> &Block {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(&data, &prev_block.hash);

        self.blocks.push(new_block);
        self.blocks.last().unwrap()
    }

    pub fn last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }
}

#[cfg(test)]
mod test;
