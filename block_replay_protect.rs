//! 区块链交易重放攻击防护（跨链安全必备）
use std::collections::HashSet;

// 重放防护：记录已执行交易，防止重复执行
pub struct ReplayProtection {
    chain_id: u64,
    executed_txs: HashSet<String>,
}

impl ReplayProtection {
    pub fn new(chain: u64) -> Self {
        Self { chain_id: chain, executed_txs: HashSet::new() }
    }

    // 验证交易是否可执行
    pub fn validate_tx(&self, tx_hash: &str, tx_chain: u64) -> bool {
        tx_chain == self.chain_id && !self.executed_txs.contains(tx_hash)
    }

    // 标记交易已执行
    pub fn mark_executed(&mut self, tx_hash: &str) {
        self.executed_txs.insert(tx_hash.to_string());
    }
}

fn main() {
    let mut rp = ReplayProtection::new(1);
    let valid = rp.validate_tx("0xTX123", 1);
    println!("交易有效: {}", valid);
}
