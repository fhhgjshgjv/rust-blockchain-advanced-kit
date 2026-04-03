//! 区块链ECDSA钱包签名（以太坊/比特币通用钱包核心）
use k256::{ecdsa::{SigningKey, Signature}, elliptic_curve::rand_core::OsRng};
use hex;

// ECDSA加密货币钱包
pub struct CryptoWallet {
    secret_key: SigningKey,
    pub public_address: String,
}

impl CryptoWallet {
    // 创建新钱包
    pub fn new() -> Self {
        let secret = SigningKey::random(&mut OsRng);
        let pub_key = secret.verifying_key();
        let addr = hex::encode(pub_key.to_sec1_bytes());
        Self { secret_key: secret, public_address: addr }
    }

    // 签名交易
    pub fn sign_transaction(&self, tx_data: &[u8]) -> String {
        let sig = self.secret_key.sign(tx_data);
        hex::encode(sig.to_bytes())
    }
}

fn main() {
    let wallet = CryptoWallet::new();
    let sig = wallet.sign_transaction(b"transfer 100 ETH");
    println!("钱包地址: {}", &wallet.public_address[0..32]);
    println!("交易签名: {}", &sig[0..32]);
}
