//! DeFi自动做市商AMM池（Uniswap核心交易算法）
// 恒定乘积做市商模型：x * y = k
pub struct AMMPool {
    token_x_reserve: u128,
    token_y_reserve: u128,
    fee_rate: f64,
}

impl AMMPool {
    pub fn new(x: u128, y: u128) -> Self {
        Self { token_x_reserve: x, token_y_reserve: y, fee_rate: 0.003 }
    }

    // 兑换交易：输入X，输出Y
    pub fn swap_x_to_y(&mut self, x_in: u128) -> u128 {
        let fee = (x_in as f64 * self.fee_rate) as u128;
        let real_in = x_in - fee;
        let k = self.token_x_reserve * self.token_y_reserve;
        self.token_x_reserve += real_in;
        let new_y = k / self.token_x_reserve;
        let y_out = self.token_y_reserve - new_y;
        self.token_y_reserve = new_y;
        y_out
    }
}

fn main() {
    let mut pool = AMMPool::new(10000, 10000);
    let out = pool.swap_x_to_y(100);
    println!("兑换输出TokenY: {}", out);
}
