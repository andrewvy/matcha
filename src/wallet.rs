use rust_sodium::crypto::sign;

#[derive(Clone, Debug)]
pub struct Address {
    pub public_key: sign::PublicKey,
    pub private_key: sign::SecretKey,
    pub amount: i64
}

pub fn create_address() -> Address {
    let (pk, sk) = sign::gen_keypair();

    Address {
        public_key: pk,
        private_key: sk,
        amount: 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_address_starts_with_empty_amount() {
        let address = create_address();

        assert_eq!(address.amount, 0);
    }
}
