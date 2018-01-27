use rust_sodium::crypto::sign;
use rust_sodium::crypto::hash;
use failure::Error;
use hex;

use config;
use matcha_pb::{Wallet, WalletKeypair};

#[derive(Debug)]
pub enum AddressNetworkIdentifier {
    PRODNET = 0x36,
    TESTNET = 0x00,
}

// Address Format:
// NETWORK_IDENTIFIER <> sha256 hash of the public key

pub const ADDRESS_LEN: usize = hash::sha256::DIGESTBYTES + 1;

pub struct Address(pub [u8; ADDRESS_LEN]);

impl Address {
    pub fn to_hex(&self) -> String {
        hex::encode(self.0.to_vec())
    }

    pub fn get_network_type(&self) -> AddressNetworkIdentifier {
        match self.0[0] {
            n if n == AddressNetworkIdentifier::PRODNET as u8 => AddressNetworkIdentifier::PRODNET,
            n if n == AddressNetworkIdentifier::TESTNET as u8 => AddressNetworkIdentifier::TESTNET,
            // @todo(vy): Really should be an error here.
            _ => AddressNetworkIdentifier::TESTNET,
        }
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
    fn get_address_struct(&self) -> Result<Address, Error>;
}

impl WalletKeypairExt for WalletKeypair {
    fn get_address_struct(&self) -> Result<Address, Error> {
        let mut address: [u8; ADDRESS_LEN] = [0; ADDRESS_LEN];
        let network_type = self.get_network_type();
        let public_key = self.get_public_key();

        if self.get_public_key().len() == sign::PUBLICKEYBYTES {
            let digest = hash::sha256::hash(public_key);

            address[0] = network_type as u8;
            for i in 0..digest.0.len() {
                address[i+1] = digest.0[i];
            }

            Ok(Address(address))
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
            keypair.set_network_type(AddressNetworkIdentifier::PRODNET as u32);

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
                let address = keypair.get_address_struct().expect("No public id");

                println!("Address [{:?}] {:?}: {:?}", address.get_network_type(), keypair.get_name(), address.to_hex());
            })
        }
        Err(_) => {
            println!("Could not create wallet");
        }
    }
}
