use rust_sodium::crypto::hash::sha256;
use bytes::BytesMut;

#[allow(unused_imports)]
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

use matcha_pb::Transaction;

/*
 * A transaction is valid if all these conditions are true:
 * - InputTransactions must reference unspent OutputTransactions (UTXOs, unspect transaction outputs)
 * - The InputTransactions total amount must be greater or equal to the output money
 * - Number of InputTransactions must be greater than 0
 * - Number of OutputTransactions must be greater than 0
 */

pub trait TransactionExtension {
    fn to_hash(&self) -> sha256::Digest;
    fn is_valid(&self) -> bool;
}

impl TransactionExtension for Transaction {
    fn to_hash(&self) -> sha256::Digest {
        let mut buffer = BytesMut::new();

        buffer.extend_from_slice(self.get_id());

        self.get_txins().iter().for_each(|txin| {
            buffer.extend_from_slice(&txin.get_tx_id());

            let mut wtr = vec![];
            wtr.write_u32::<LittleEndian>(txin.get_txout_index()).unwrap();
            buffer.extend_from_slice(wtr.as_slice());

            buffer.extend_from_slice(&txin.get_signature());
            buffer.extend_from_slice(&txin.get_public_key());
        });

        self.get_txouts().iter().for_each(|txout| {
            let mut wtr = vec![];
            wtr.write_u64::<LittleEndian>(txout.get_amount()).unwrap();
            buffer.extend_from_slice(wtr.as_slice());
            buffer.extend_from_slice(&[0; 32]);
        });

        sha256::hash(buffer.to_vec().as_slice())
    }

    fn is_valid(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matcha_pb::InputTransaction;
    use hex;

    #[test]
    fn can_hash_empty_transaction() {
        // A new completely-empty transaction is 32 bytes of zeros.
        let transaction = Transaction::new();

        assert_eq!(
          hex::encode(transaction.to_hash()),
          "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn can_hash_transaction_with_txins() {
        let mut transaction = Transaction::new();
        let txin = InputTransaction::new();

        &transaction.txins.push(txin);

        assert_eq!(
          hex::encode(transaction.to_hash()),
          "df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119"
        );
    }
}
