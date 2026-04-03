//! 区块链验证者委员会管理（联盟链/PoS链节点权限核心）
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub is_active: bool,
}

// 验证者委员会：管理出块节点权限
pub struct ValidatorCommittee {
    validators: Vec<Validator>,
    active_set: HashSet<String>,
}

impl ValidatorCommittee {
    pub fn new() -> Self {
        Self { validators: vec![], active_set: HashSet::new() }
    }

    // 注册验证者
    pub fn register_validator(&mut self, address: &str, stake: u64) {
        let is_active = stake >= 1000;
        self.validators.push(Validator {
            address: address.to_string(),
            stake,
            is_active,
        });
        if is_active { self.active_set.insert(address.to_string()); }
    }

    // 获取活跃验证者列表
    pub fn active_validators(&self) -> Vec<&String> {
        self.active_set.iter().collect()
    }
}

fn main() {
    let mut committee = ValidatorCommittee::new();
    committee.register_validator("0xNodeA", 5000);
    println!("活跃验证者: {:?}", committee.active_validators());
}
