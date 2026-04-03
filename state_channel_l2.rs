//! 区块链Layer2状态通道（链下扩容、零Gas高频交易）
use std::collections::HashMap;

// 链下状态通道：实现链下交易，最终上链结算
pub struct StateChannel {
    channel_id: String,
    parties: (String, String),
    balances: HashMap<String, u64>,
    is_closed: bool,
}

impl StateChannel {
    pub fn open(id: &str, party_a: &str, party_b: &str, a_balance: u64, b_balance: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert(party_a.to_string(), a_balance);
        balances.insert(party_b.to_string(), b_balance);
        Self {
            channel_id: id.to_string(),
            parties: (party_a.to_string(), party_b.to_string()),
            balances,
            is_closed: false,
        }
    }

    // 链下转账
    pub fn offchain_transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        if self.is_closed { return false; }
        let from_bal = self.balances.get(from).cloned().unwrap_or(0);
        if from_bal < amount { return false; }
        *self.balances.get_mut(from).unwrap() -= amount;
        *self.balances.get_mut(to).unwrap() += amount;
        true
    }
}

fn main() {
    let mut channel = StateChannel::open("channel_01", "Alice", "Bob", 1000, 1000);
    let res = channel.offchain_transfer("Alice", "Bob", 200);
    println!("链下交易成功: {}", res);
}
