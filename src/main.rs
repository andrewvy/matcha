extern crate rust_sodium;

mod wallet;

fn main() {
    let address = wallet::create_address();

    println!("Public Key: {:?}", address.public_key);
    println!("Private Key: {:?}", address.private_key);
    println!("Amount: {:?}", address.amount);
}
