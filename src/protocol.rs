use std::time::{SystemTime, UNIX_EPOCH};
use std::net::SocketAddr;

use rust_sodium::crypto::sign;
use rust_sodium::crypto::hash::sha256;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    pub keypairs: Vec<WalletKeypair>
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            keypairs: Vec::new()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WalletKeypair {
    pub name: String,
    pub public_key: sign::PublicKey,
    pub secret_key: sign::SecretKey,
}

#[allow(dead_code)]
pub struct InputTransaction {
    // - InputTransaction Header
    pub tx_id: sha256::Digest,
    pub txout_index: u32,
    // - ///

    pub signature: sign::Signature,
    pub public_key: sign::PublicKey,
}

#[allow(dead_code)]
pub struct OutputTransaction {
    pub amount: u64,
    pub public_key: sign::PublicKey,
}

#[allow(dead_code)]
pub struct Transaction {
    pub id: sha256::Digest,
    pub txins: Vec<InputTransaction>,
    pub txouts: Vec<OutputTransaction>,
}

#[allow(dead_code)]
pub struct Block {
    pub hash: sha256::Digest,
    pub previous_hash: sha256::Digest,
    pub transaction_root: sha256::Digest,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

#[allow(dead_code)]
impl Block {
    pub fn new() -> Block {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is behind UNIX_EPOCH!");

        Block {
            hash: sha256::Digest([0; 32]),
            previous_hash: sha256::Digest([0; 32]),
            transaction_root: sha256::Digest([0; 32]),
            timestamp: current_time.as_secs(),
            transactions: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Ping((Uuid, SocketAddr)),
    Pong((Uuid, SocketAddr)),
    PeerList(Vec<(Uuid, SocketAddr)>),
}

#[allow(dead_code)]
#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "file not found: {}", name)]
    FileNotFound {
        name: String
    }
}

#[allow(dead_code)]
#[derive(Debug, Fail)]
pub enum ProtocolError {
    #[fail(display = "invalid message")]
    InvalidMessage {},

    #[fail(display = "could not serialize message")]
    SerializeError {}
}
