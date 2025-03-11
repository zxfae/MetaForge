use super::Block;

#[test]
fn block_format() {
    let datas = vec!["Hello World !".to_string()];
    let prev_hash = [1u8; 32];
    let block = Block::new(&datas, &prev_hash);

    assert_eq!(block.data, datas);
}
