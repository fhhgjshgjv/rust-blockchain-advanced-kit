//! 区块链区块归档存储（全节点数据持久化）
use std::collections::HashMap;
use sha2::Sha256;

// 区块归档：永久存储区块数据，支持快速查询
pub struct BlockArchive {
    blocks: HashMap<u64, String>, // 高度 => 区块哈希
    block_data: HashMap<String, String>, // 哈希 => 数据
}

impl BlockArchive {
    pub fn new() -> Self {
        Self { blocks: HashMap::new(), block_data: HashMap::new() }
    }

    // 归档区块
    pub fn archive_block(&mut self, height: u64, hash: &str, data: &str) {
        self.blocks.insert(height, hash.to_string());
        self.block_data.insert(hash.to_string(), data.to_string());
    }

    // 根据高度获取区块
    pub fn get_block_by_height(&self, height: u64) -> Option<&String> {
        self.blocks.get(&height)
    }
}

fn main() {
    let mut archive = BlockArchive::new();
    archive.archive_block(100, "0xBLOCK100", "txs:5,time:1712345678");
    println!("区块100哈希: {:?}", archive.get_block_by_height(100));
}
