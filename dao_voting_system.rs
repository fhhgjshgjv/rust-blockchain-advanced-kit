//! 区块链DAO去中心化投票（链上治理核心）
use std::collections::HashMap;

// DAO投票系统：按权重投票，自动统计结果
pub struct DaoVoting {
    proposal_id: String,
    votes_for: u64,
    votes_against: u64,
    voted: HashMap<String, bool>,
}

impl DaoVoting {
    pub fn new(id: &str) -> Self {
        Self {
            proposal_id: id.to_string(),
            votes_for: 0,
            votes_against: 0,
            voted: HashMap::new(),
        }
    }

    // 投票：weight=投票权重(代币数量)
    pub fn vote(&mut self, voter: &str, approve: bool, weight: u64) -> bool {
        if self.voted.contains_key(voter) { return false; }
        self.voted.insert(voter.to_string(), true);
        if approve { self.votes_for += weight; } else { self.votes_against += weight; }
        true
    }
}

fn main() {
    let mut voting = DaoVoting::new("proposal_001");
    voting.vote("user1", true, 1000);
    println!("赞成票: {}", voting.votes_for);
}
