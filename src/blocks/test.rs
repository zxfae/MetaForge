use super::Block;

//@Dev : Block::new() get Vec<String> && [u8; 32] parameters;
//@Dev : Block.hash is [u8; 32]
//@Notice : Block creation; datas correctly set; hash_calculation(assert_ne!)
#[test]
fn block_format() {
    let datas: Vec<String> = vec!["Hello World !".to_string()];
    let prev_hash: [u8; 32] = [1u8; 32];
    let block = Block::new(&datas, &prev_hash);

    assert_eq!(block.data, datas);
    assert_eq!(block.prev_hash, prev_hash);

    assert_ne!(block.hash, [0u8; 32]);
}

//@Dev : Data block data && hash values set at genesis_bloc()
//@Dev : Created on mod.rs
#[test]
fn genesis_block() {
    let genesis_block = Block::genesis_block();

    assert_eq!(genesis_block.data, vec!["Genesis_Block !".to_string()]);
    assert_eq!(genesis_block.prev_hash, [0u8; 32]);

    assert_ne!(genesis_block.hash, [0u8; 32])
}

//@Notice : Compare same hashblock for test hash_calculate function
//@Dev : with other data; hash must be != equal
#[test]
fn test_hash() {
    let data: Vec<String> = vec!["Hello World !".to_string()];
    let prev_hash: [u8; 32] = [2u8; 32];
    let block1 = Block::new(&data, &prev_hash);
    // Will be same
    let hash_calculated = block1.calculate_hash();
    assert_eq!(block1.hash, hash_calculated);

    let other_data = vec!["Others datas !".to_string()];
    let block2 = Block::new(&other_data, &prev_hash);

    assert_ne!(block1.hash, block2.hash);
}

#[test]
fn test_blockchain_block() {
    let genesis = Block::genesis_block();
    let next_data: Vec<String> = vec!["Next Block".to_string()];

    let next_block = Block::new(&next_data, &genesis.hash);
    //@Dev : nextblock.prev must be equal to genesis hash
    assert_eq!(next_block.prev_hash, genesis.hash);
    assert_ne!(genesis.hash, next_block.hash);
}
