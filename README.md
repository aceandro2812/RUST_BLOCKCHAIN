<h1>Create a Simple Blockchain in Rust</h1>

<p>If you're a novice Rust developer learning Rust, you can follow these steps to create a simple blockchain in Rust. The instructions below assume you have Rust and Cargo installed on your machine.</p>

<h2>1. Set Up the Project</h2>

<p>Navigate to the desired directory on your computer using the terminal or command prompt:</p>

<pre><code>cd D:\rust_projects</code></pre>

<p>Create a new Rust project in the selected directory:</p>

<pre><code>cargo new simple_blockchain</code></pre>

<h2>2. Add Dependencies</h2>

<p>Open the <code>Cargo.toml</code> file within the newly created <code>simple_blockchain</code> directory. Add the following dependencies:</p>

<pre><code>[dependencies]
chrono = "0.4"
sha2 = "0.9.5"</code></pre>

<h2>3. Implement Blockchain Logic</h2>

<p>Create a new file called <code>main.rs</code> within the <code>src</code> directory. Replace the default content with the following code:</p>

<pre><code>extern crate chrono;
extern crate sha2;

use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, timestamp: i64, data: String, previous_hash: String) -> Block {
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u32, timestamp: i64, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

fn main() {
    let mut blockchain: Vec<Block> = Vec::new();

    // Genesis block
    let genesis_block = Block::new(0, Utc::now().timestamp(), "Genesis Block".to_string(), "0".to_string());
    blockchain.push(genesis_block);

    let block1 = Block::new(1, Utc::now().timestamp(), "Data for Block 1".to_string(), blockchain[0].hash.clone());
    blockchain.push(block1);

    let block2 = Block::new(2, Utc::now().timestamp(), "Data for Block 2".to_string(), blockchain[1].hash.clone());
    blockchain.push(block2);

    println!("{:#?}", blockchain);
}</code></pre>

<h2>4. Run the Code</h2>

<p>In the terminal, navigate to the project directory:</p>

<pre><code>cd simple_blockchain</code></pre>

<p>Run the code:</p>

<pre><code>cargo run</code></pre>

<p>You should see the blockchain printed in a formatted way, displaying each block's information.</p>

<h2>Conclusion</h2>

<p>Feel free to experiment with the code and explore additional features. This serves as a basic starting point for your Rust blockchain project. Have fun learning and coding in Rust!</p>
