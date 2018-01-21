use rust_sodium::crypto::sign;
use rust_sodium::crypto::hash;
use failure::Error;
use hex;

use config;
use matcha_pb::{Wallet, WalletKeypair};

pub const PUBLIC_ID_LEN: usize = 32;

pub struct PublicId(pub [u8; PUBLIC_ID_LEN]);

impl PublicId {
    pub fn to_hex(&self) -> String {
        hex::encode(self.0.to_vec())
    }
}

pub trait WalletExt {
    fn add_keypair(&mut self, keypair: WalletKeypair);
}

impl WalletExt for Wallet {
    fn add_keypair(&mut self, keypair: WalletKeypair) {
        self.mut_keypairs().push(keypair);
    }
}

pub trait WalletKeypairExt {
    fn get_public_id_struct(&self) -> Result<PublicId, Error>;
}

impl WalletKeypairExt for WalletKeypair {
    fn get_public_id_struct(&self) -> Result<PublicId, Error> {
        let public_key = self.get_public_key();

        if self.get_public_key().len() == sign::PUBLICKEYBYTES {
            let public_id = hash::sha256::hash(public_key);
            let public_id_struct = PublicId(public_id.0);
            Ok(public_id_struct)
        } else {
            Err(format_err!("Invalid public id"))
        }
    }
}

pub fn init_wallet() {
    let result: Result<Wallet, Error> = config::load_proto_from_file("wallet.dat");

    match result {
        Ok(_) => println!("Wallet already created"),
        Err(_) => {
            let mut wallet = Wallet::new();

            config::save_proto_to_file("wallet.dat", &wallet)
                .expect("Could not save wallet");

            println!("Created wallet");
        }
    }
}

pub fn load_or_create_wallet() -> Result<Wallet, Error> {
    let result: Result<Wallet, Error> = config::load_proto_from_file("wallet.dat");

    match result {
        Ok(wallet) => Ok(wallet),
        Err(_) => {
            let mut wallet = Wallet::new();

            match config::save_proto_to_file("wallet.dat", &wallet) {
                Ok(_) => Ok(wallet),
                Err(_) => Err(format_err!("Could not create wallet")),
            }
        }
    }
}

pub fn save_wallet(wallet: Wallet) {
    config::save_proto_to_file("wallet.dat", &wallet)
        .expect("Could not save wallet");
}

pub fn create_new_address(name: &str) {
    match load_or_create_wallet() {
        Ok(mut wallet) => {
            let mut keypair = WalletKeypair::new();
            let (pk, sk) = sign::gen_keypair();

            keypair.set_public_key(pk.0.to_vec());
            keypair.set_secret_key(sk.0.to_vec());
            keypair.set_name(name.to_owned());

            wallet.add_keypair(keypair);
            save_wallet(wallet);
            println!("Created new address named '{}'", name);
        },
        Err(_) => {
            println!("Could not create address {}", name);
        }
    }
}

pub fn list_wallet() {
    match load_or_create_wallet() {
        Ok(wallet) => {
            wallet.get_keypairs().iter().for_each(|keypair| {
                println!("Address {:?}: {:?}", keypair.get_name(), keypair.get_public_id_struct().expect("No public id").to_hex());
            })
        }
        Err(_) => {
            println!("Could not create wallet");
        }
    }
}
