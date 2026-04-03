//! 区块链创世区块构建（所有公链第一条区块）
use sha2::Sha256;
use hex;

// 创世区块：区块链的第一个区块，无前置哈希
#[derive(Debug)]
pub struct GenesisBlock {
    height: u64,
    hash: String,
    timestamp: u64,
    initial_supply: u128,
}

impl GenesisBlock {
    pub fn create(timestamp: u64, supply: u128) -> Self {
        let data = format!("genesis_rust_chain_{}_{}", timestamp, supply);
        let hash = hex::encode(Sha256::digest(data.as_bytes()));
        Self {
            height: 0,
            hash,
            timestamp,
            initial_supply: supply,
        }
    }
}

fn main() {
    let genesis = GenesisBlock::create(1712345678, 21000000);
    println!("创世区块: {:?}", genesis);
}
