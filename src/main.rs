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

    println!("test, {:?}", my_own_blockchain);

    let sim_n = 10;
    for i in 1..=sim_n {
        thread::sleep(Duration::from_secs(3));

        let data = format!("Data for block {}", i);
        let new_block = my_own_blockchain.add(data);

        println!("\nBloc #{} added", i);
        println!("  Header: {}", new_block.header);
        println!("  Hash: {}", new_block.hash_to_hex());
        println!(
            "  Timestamp: {}",
            new_block.formatted_timestamp_with_month()
        );
        println!("  Prev_block: {}", hex::encode(new_block.prev_hash));
    }

    let is_ok = my_own_blockchain.verify();
    println!("\nValidity: {}", is_ok);

    println!("\nTesting blockchain integrity");
    let mut tampered_blockchain = my_own_blockchain;
    tampered_blockchain.blocks[1].header = "Block #Data for block 2".to_string();
    let is_still_valid = tampered_blockchain.verify();
    println!("Blockchain validity after tampering: {}", is_still_valid);
}
