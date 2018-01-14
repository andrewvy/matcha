use rust_sodium::crypto::sign;
use rust_sodium::crypto::hash;
use hex;

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
    pub amount: i64
}

pub fn create_address() -> Address {
    let (pk, sk) = sign::gen_keypair();

    Address {
        public_id: PublicId(hash::sha256::hash(&pk[..]).0),
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
