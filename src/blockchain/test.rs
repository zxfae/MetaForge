use super::Blockchain;

#[test]
fn new_blockchain() {
    let blockchain = Blockchain::new();

    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(
        blockchain.blocks[0].data,
        vec!["Genesis_Block !".to_string()]
    );
}

//@Dev : Test adding function logical
#[test]
fn add_block_test() {
    let mut blockchain = Blockchain::new();
    let data: Vec<String> = vec!["Test add block".to_string()];

    //@Dev : Stock on other variable; block[0] hash
    let genesis_block_hash = blockchain.blocks[0].hash;
    let genesis_timestamp = blockchain.blocks[0].time_stamp;
    let genesis_prev_hash = blockchain.blocks[0].prev_hash;
    //@Dev : Call add function
    let new_block = blockchain.add(&data);

    assert_eq!(new_block.prev_hash, genesis_block_hash);
    assert_ne!(new_block.hash, genesis_block_hash);
    //@Dev : Ensure time_stamp are different between blocks
    assert_ne!(new_block.time_stamp, genesis_timestamp);
    assert_ne!(new_block.prev_hash, genesis_prev_hash)
}
