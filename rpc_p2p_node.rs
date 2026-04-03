//! 区块链P2P+RPC节点通信（节点间数据同步协议）
use std::collections::VecDeque;

// P2P节点：区块链网络通信核心
pub struct P2pRpcNode {
    node_id: String,
    peer_list: Vec<String>,
    message_queue: VecDeque<String>,
}

impl P2pRpcNode {
    pub fn new(id: &str) -> Self {
        Self {
            node_id: id.to_string(),
            peer_list: vec![],
            message_queue: VecDeque::new(),
        }
    }

    // 添加邻居节点
    pub fn add_peer(&mut self, peer_id: &str) {
        self.peer_list.push(peer_id.to_string());
    }

    // 广播区块消息
    pub fn broadcast_block(&mut self, block_hash: &str) {
        let msg = format!("NEW_BLOCK:{}", block_hash);
        self.message_queue.push_back(msg);
    }
}

fn main() {
    let mut node = P2pRpcNode::new("node_rust_01");
    node.add_peer("node_eth_02");
    node.broadcast_block("0xBROADCAST");
    println!("节点消息队列: {:?}", node.message_queue);
}
