
use rust_sodium::crypto::sign;
use rust_sodium::crypto::hash;
use hex;
use protobuf;

use config;
use matcha_pb::{Wallet, WalletKeypair};

pub trait WalletExtension {
    fn add_keypair(&mut self, name: &str, public_key: &sign::PublicKey, secret_key: &sign::SecretKey);
}

impl WalletExtension for Wallet {
    fn add_keypair(&mut self, name: &str, public_key: &sign::PublicKey, secret_key: &sign::SecretKey) {
        let mut key = WalletKeypair::new();
        key.set_public_key(public_key.0.to_vec());
        key.set_secret_key(secret_key.0.to_vec());
        key.set_name(String::from(name));
        self.mut_keypairs().push(key);
    }
}

/// Constant byte length of `PublicId`.
pub const PUBLIC_ID_LEN: usize = 32;

/// Constant bit length of `PublicId`.
#[allow(dead_code)]
pub const PUBLIC_ID_BITS: usize = PUBLIC_ID_LEN * 8;

#[derive(Clone, Debug)]
pub struct PublicId(pub [u8; PUBLIC_ID_LEN]);

impl PublicId {
    pub fn to_hex(&self) -> String {
        hex::encode(self.0.to_vec())
    }
}

#[derive(Clone, Debug)]
pub struct Address {
    pub public_id: PublicId,
    pub public_key: sign::PublicKey,
    pub private_key: sign::SecretKey,
}

pub fn load_wallet_from_file() -> Result<Wallet, protobuf::ProtobufError> {
    config::load_proto_from_file("wallet.dat")
}

pub fn save_wallet_to_file(wallet: Wallet) -> Result<(), protobuf::ProtobufError> {
    config::save_proto_to_file("wallet.dat", &wallet)
}

pub fn create_wallet() {
    match load_wallet_from_file() {
        Err(_) => {
            let mut wallet = Wallet::new();
            let address = create_address();

            wallet.add_keypair("default", &address.public_key, &address.private_key);

            match save_wallet_to_file(wallet) {
                Err(_) => println!("Could not create wallet"),
                Ok(_) => {
                    println!("Created wallet successfully!");
                    println!("---");
                    println!("Keypair Name: {}", "default");
                    println!("Public ID: {}", address.public_id.to_hex());
                    println!("Public Key: {:?}", address.public_key);
                    println!("Private Key: {:?}", address.private_key);
                }
            }
        },
        Ok(_wallet) => {
            println!("Wallet already created!")
        }
    }
}

pub fn create_address() -> Address {
    let (pk, sk) = sign::gen_keypair();

    Address {
        public_id: PublicId(hash::sha256::hash(&pk[..]).0),
        public_key: pk,
        private_key: sk,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
