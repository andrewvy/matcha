use rust_sodium::crypto::sign;
use rust_sodium::crypto::hash;
use failure::Error;
use hex;

use config;

use protocol::{Wallet, WalletKeypair};

pub trait WalletExtension {
    fn add_keypair(&mut self, name: &str, public_key: &sign::PublicKey, secret_key: &sign::SecretKey);
    fn print_keypairs(&self);
}

impl WalletExtension for Wallet {
    fn add_keypair(&mut self, name: &str, public_key: &sign::PublicKey, secret_key: &sign::SecretKey) {
        let key = WalletKeypair {
            name: String::from(name),
            public_key: public_key.clone(),
            secret_key: secret_key.clone()
        };

        self.keypairs.push(key);
    }

    fn print_keypairs(&self) {
        self.keypairs.iter().for_each(|keypair| {
            let name = &keypair.name;
            let public_key = &keypair.public_key;
            let secret_key = &keypair.secret_key;

            println!("-- Keypair: {}", name);
            println!("Public Key: {:?}", public_key);
            println!("Secret Key: {:?}", secret_key);
        })
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
    #[allow(dead_code)]
    pub fn to_hex(&self) -> String {
        hex::encode(self.0.to_vec())
    }
}

#[derive(Clone, Debug)]
pub struct Address {
    pub public_id: PublicId,
    pub public_key: sign::PublicKey,
    pub secret_key: sign::SecretKey,
}

pub fn load_wallet_from_file() -> Result<Wallet, Error> {
    config::load_bincode_from_file("wallet.dat")
}

pub fn save_wallet_to_file(wallet: Wallet) -> Result<(), Error> {
    config::save_bincode_to_file("wallet.dat", &wallet)
}

pub fn create_wallet() {
    match load_wallet_from_file() {
        Err(_) => {
            let mut wallet = Wallet::new();
            let address = create_address();

            wallet.add_keypair("default", &address.public_key, &address.secret_key);

            match save_wallet_to_file(wallet) {
                Err(_) => println!("Could not create wallet"),
                Ok(_) => {
                    println!("Created wallet successfully!");
                    println!("---");
                    println!("Keypair Name: {}", "default");
                    println!("Public Key: {:?}", address.public_key);
                    println!("Secret Key: {:?}", address.secret_key);
                    println!("---");
                }
            }
        },
        Ok(wallet) => {
            println!("Wallet already created!");
            wallet.print_keypairs();
        }
    }
}

pub fn list_wallet() {
    match load_wallet_from_file() {
        Err(e) => {
            println!("Error: {}", e);
        },
        Ok(wallet) => {
            wallet.print_keypairs();
        }
    }
}

pub fn create_address() -> Address {
    let (pk, sk) = sign::gen_keypair();

    Address {
        public_id: PublicId(hash::sha256::hash(&pk[..]).0),
        public_key: pk,
        secret_key: sk,
    }
}
