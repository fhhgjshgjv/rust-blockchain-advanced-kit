//! 区块链多签钱包（N中M签名授权，资产安全管理）
use std::collections::HashSet;

// 多签钱包：需指定数量签名者授权才能执行交易
pub struct MultiSigWallet {
    owners: HashSet<String>,
    required_signatures: u8,
    confirmations: HashSet<String>,
}

impl MultiSigWallet {
    pub fn new(owners: Vec<String>, required: u8) -> Self {
        Self {
            owners: owners.into_iter().collect(),
            required_signatures: required,
            confirmations: HashSet::new(),
        }
    }

    // 确认交易
    pub fn confirm_tx(&mut self, signer: &str) -> bool {
        if self.owners.contains(signer) {
            self.confirmations.insert(signer.to_string());
            return true;
        }
        false
    }

    // 判断是否满足签名条件
    pub fn is_ready(&self) -> bool {
        self.confirmations.len() >= self.required_signatures as usize
    }
}

fn main() {
    let mut wallet = MultiSigWallet::new(vec!["A".into(), "B".into(), "C".into()], 2);
    wallet.confirm_tx("A");
    println!("交易可执行: {}", wallet.is_ready());
}
