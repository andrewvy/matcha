use rust_sodium::crypto::hash::sha256;
use bytes::BytesMut;
use byteorder::{ByteOrder, LittleEndian};
use hex;

use protocol::Transaction;


pub trait TransactionExtension {
    fn to_hash(&self) -> sha256::Digest;
}

impl TransactionExtension for Transaction {
    fn to_hash(&self) -> sha256::Digest {
        let mut buffer = BytesMut::new();

        buffer.extend_from_slice(&self.id.0);

        self.txins.iter().for_each(|txin| {
            buffer.extend_from_slice(&txin.tx_id.0);
            LittleEndian::write_u32(&mut buffer, txin.txout_index);
            buffer.extend_from_slice(&txin.signature.0);
            buffer.extend_from_slice(&txin.public_key.0);
        });

        self.txouts.iter().for_each(|txout| {
            LittleEndian::write_u64(&mut buffer, txout.amount);
            buffer.extend_from_slice(&txout.public_key.0);
        });

        sha256::hash(buffer.to_vec().as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hash_transaction() {
        // A new transaction is 32 bytes of zeros.
        let transaction = Transaction::new();

        assert_eq!(
          hex::encode(transaction.to_hash()),
          "66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925"
        );
    }
}
