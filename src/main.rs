mod block;
mod blockchain;

use blockchain::Blockchain;
use std::io;

fn main() {
    let mut blockchain = Blockchain::new();

    loop {
        println!("\n===== SIMPLE BLOCKCHAIN =====");
        println!("1. Add Block");
        println!("2. Show Blockchain");
        println!("3. Validate Blockchain");
        println!("4. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter block data:");

                let mut data = String::new();

                io::stdin()
                    .read_line(&mut data)
                    .expect("Failed to read data");

                blockchain.add_block(
                    data.trim().to_string(),
                );

                println!("Block added successfully.");
            }

            "2" => {
                blockchain.display_chain();
            }

            "3" => {
                if blockchain.validate_chain() {
                    println!("Blockchain is VALID");
                } else {
                    println!("Blockchain is INVALID");
                }
            }

            "4" => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("Invalid option");
            }
        }
    }
}