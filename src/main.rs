pub mod blockchain;
pub mod blocks;
pub mod utils;

use std::thread;
use std::time::Duration;

use blockchain::Blockchain;

fn main() {
    let mut my_own_blockchain = Blockchain::new();

    println!(
        "Genesis block hash {}\n",
        my_own_blockchain.blocks[0].hash_to_hex()
    );

    println!("Genesis block{:?}\n", my_own_blockchain);

    let simulation = 10;
    for i in 1..=simulation {
        thread::sleep(Duration::from_secs(3));

        let data = vec!["Hello World !".to_string()];
        let new_block = my_own_blockchain.add(data);

        println!("\nBloc #{} added", i);
        println!("  Datas: {:?}", new_block.data);
        println!("  Hash: {}", new_block.hash_to_hex());
        println!(
            "  Timestamp: {}",
            new_block.formatted_timestamp_with_month()
        );
        println!("  Prev_block: {}", hex::encode(new_block.prev_hash));
    }
}
