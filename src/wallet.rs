use rust_sodium::crypto::sign::{self, PublicKey, SecretKey};
use failure::Error;

use config;
use matcha_pb::{Wallet, WalletKeypair};

pub trait WalletExt {
    fn add_keypair(&mut self, name: &str, public_key: &PublicKey, secret_key: &SecretKey);
}

impl WalletExt for Wallet {
    fn add_keypair(&mut self, name: &str, public_key: &PublicKey, secret_key: &SecretKey) {
        let mut keypair = WalletKeypair::new();

        keypair.set_public_key(public_key.0.to_vec());
        keypair.set_secret_key(secret_key.0.to_vec());
        keypair.set_name(name.to_owned());

        self.mut_keypairs().push(keypair);
    }
}

pub fn create_wallet() {
    let result: Result<Wallet, Error> = config::load_proto_from_file("wallet.dat");

    match result {
        Ok(_) => println!("Wallet already created"),
        Err(_) => {
            let mut wallet = Wallet::new();
            let (pk, sk) = sign::gen_keypair();
            let name = "default";

            wallet.add_keypair(&name, &pk, &sk);

            config::save_proto_to_file("wallet.dat", &wallet)
                .expect("Could not save wallet");

            println!("Created wallet");
        }
    }
}
