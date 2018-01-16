use rust_sodium::crypto::sign;

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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Object {
    Wallet(Wallet),
    WalleyKeypair(WalletKeypair)
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "file not found: {}", name)]
    FileNotFound {
        name: String
    }
}

#[derive(Debug, Fail)]
pub enum ProtocolError {
    #[fail(display = "invalid message")]
    InvalidMessage {},

    #[fail(display = "could not serialize message")]
    SerializeError {}
}

