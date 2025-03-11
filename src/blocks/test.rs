use sha2::digest::typenum::assert_type;

use super::Block;

//@Notice : Block creation; datas correctly set; hash_calculation(assert_ne!)
#[test]
fn block_format() {
    let datas = vec!["Hello World !".to_string()];
    let prev_hash = [1u8; 32];
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
