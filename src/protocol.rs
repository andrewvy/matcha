use std::time::{SystemTime, UNIX_EPOCH};

use rust_sodium::crypto::sign;
use rust_sodium::crypto::hash::sha256;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputTransaction {
    // - InputTransaction Header
    pub tx_id: sha256::Digest, // 32-bytes
    pub txout_index: u32, // 4-bytes
    // - ///

    pub signature: sign::Signature, // 64-bytes
    pub public_key: sign::PublicKey, // 32-bytes
}

#[allow(dead_code)]
impl InputTransaction {
    pub fn new() -> InputTransaction {
        InputTransaction {
            tx_id: sha256::Digest([0; sha256::DIGESTBYTES]),
            txout_index: 0,
            signature: sign::Signature([0; sign::SIGNATUREBYTES]),
            public_key: sign::PublicKey([0; sign::PUBLICKEYBYTES]),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputTransaction {
    pub amount: u64,
    pub public_key: sign::PublicKey,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: sha256::Digest, // 32-bytes
    pub txins: Vec<InputTransaction>,
    pub txouts: Vec<OutputTransaction>,
}

#[allow(dead_code)]
impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            id: sha256::Digest([0; 32]),
            txins: Vec::new(),
            txouts: Vec::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[allow(dead_code)]
#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "file not found: {}", name)]
    FileNotFound {
        name: String
    }
}
