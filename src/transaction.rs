use rust_sodium::crypto::hash::sha256;
use bytes::BytesMut;

#[allow(unused_imports)]
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

use hex;

use protocol::{Transaction, InputTransaction};


pub trait TransactionExtension {
    fn to_hash(&self) -> sha256::Digest;
}

impl TransactionExtension for Transaction {
    fn to_hash(&self) -> sha256::Digest {
        let mut buffer = BytesMut::new();

        buffer.extend_from_slice(&self.id.0);

        self.txins.iter().for_each(|txin| {
            buffer.extend_from_slice(&txin.tx_id.0);

            let mut wtr = vec![];
            wtr.write_u32::<LittleEndian>(txin.txout_index).unwrap();
            buffer.extend_from_slice(wtr.as_slice());

            buffer.extend_from_slice(&txin.signature.0);
            buffer.extend_from_slice(&txin.public_key.0);
        });

        self.txouts.iter().for_each(|txout| {
            let mut wtr = vec![];
            wtr.write_u64::<LittleEndian>(txout.amount).unwrap();
            buffer.extend_from_slice(wtr.as_slice());

            buffer.extend_from_slice(&txout.public_key.0);
        });

        sha256::hash(buffer.to_vec().as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hash_empty_transaction() {
        // A new completely-empty transaction is 32 bytes of zeros.
        let transaction = Transaction::new();

        assert_eq!(
          hex::encode(transaction.to_hash()),
          "66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925"
        );
    }

    #[test]
    fn can_hash_transaction_with_txins() {
        let mut transaction = Transaction::new();
        let txin = InputTransaction::new();

        &transaction.txins.push(txin);

        assert_eq!(
          hex::encode(transaction.to_hash()),
          "4b22ee6807a2e4ddfd1bf16851609a7a009f4c6bd5aeb78162f121cb89492f02"
        );
    }
}
