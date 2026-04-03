//! 区块链Gas费优化器（自动计算最优手续费）
// 动态Gas费：根据网络拥堵自动调整
pub struct GasFeeOptimizer {
    base_fee: u64,
    priority_fee: u64,
    congestion: f64, // 0.0~1.0
}

impl GasFeeOptimizer {
    pub fn new(base: u64) -> Self {
        Self { base_fee: base, priority_fee: 2, congestion: 0.0 }
    }

    // 计算最优Gas费
    pub fn optimal_gas(&self) -> u64 {
        let congestion_factor = (1.0 + self.congestion) as u64;
        self.base_fee * congestion_factor + self.priority_fee
    }

    // 更新网络拥堵状态
    pub fn update_congestion(&mut self, level: f64) {
        self.congestion = level.clamp(0.0, 1.0);
    }
}

fn main() {
    let mut gas = GasFeeOptimizer::new(10);
    gas.update_congestion(0.7);
    println!("最优Gas费: {}", gas.optimal_gas());
}
