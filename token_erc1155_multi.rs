//! Rust实现ERC1155多标准代币（同质化+非同质化一体）
use std::collections::HashMap;

// ERC1155：一合约支持FT+NFT混合资产
pub struct ERC1155MultiToken {
    balances: HashMap<(String, u64), u64>, // (用户, TokenID) => 数量
    uri: HashMap<u64, String>,
}

impl ERC1155MultiToken {
    pub fn new() -> Self {
        Self { balances: HashMap::new(), uri: HashMap::new() }
    }

    // 批量铸造
    pub fn mint_batch(&mut self, to: &str, ids: Vec<u64>, amounts: Vec<u64>) {
        for (id, amt) in ids.into_iter().zip(amounts.into_iter()) {
            *self.balances.entry((to.to_string(), id)).or_insert(0) += amt;
        }
    }
}

fn main() {
    let mut token = ERC1155MultiToken::new();
    token.mint_batch("user1", vec![1,2], vec![100,1]);
    println!("ERC1155批量铸造完成");
}
