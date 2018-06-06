use rust_sodium::crypto::hash::sha256;
use bytes::BytesMut;
use ring::digest::Context;
use merkle::Hashable;

#[allow(unused_imports)]
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

use matcha_pb::Transaction;

use database::{self, Database, Storeable};

/*
 * A transaction is valid if all these conditions are true:
 * - InputTransactions must reference unspent OutputTransactions (UTXOs, unspect transaction outputs)
 * - The InputTransactions total amount must be greater or equal to the output money
 * - Number of InputTransactions must be greater than 0
 * - Number of OutputTransactions must be greater than 0
 */

pub trait TransactionExt {
    fn get_hash_context(&self) -> BytesMut;
    fn to_hash(&self) -> sha256::Digest;
    fn is_valid(&self, db: &Database) -> bool;
    fn txins_are_valid(&self) -> bool;
    fn txouts_are_valid(&self) -> bool;
    fn txins_reference_unspent_txouts(&self) -> bool;
    fn amount_transfer_is_valid(&self, db: &Database) -> bool;
}

impl TransactionExt for Transaction {
    fn get_hash_context(&self) -> BytesMut {
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

        buffer
    }

    fn to_hash(&self) -> sha256::Digest {
        let buffer = self.get_hash_context();

        sha256::hash(buffer.to_vec().as_slice())
    }

    fn is_valid(&self, db: &Database) -> bool {
        return self.get_txins().len() > 0 &&
            self.get_txouts().len() > 0 &&
            self.txins_are_valid() &&
            self.txouts_are_valid() &&
            self.amount_transfer_is_valid(db);
    }

    fn txins_are_valid(&self) -> bool {
        // @todo(vy): txins must reference UTXOs, and their signature
        // must be valid for the matched public key.
        return self.txins_reference_unspent_txouts();
    }

    fn txouts_are_valid(&self) -> bool {
        // @todo(vy): Depending on type of txout, that txout may
        // undergo extra validation.
        //
        // Username is ASCII, alphanumeric. 30 chars max.
        // NewPostTransaction must be 140 codepoints, NFC-normalized
        return true;
    }

    fn txins_reference_unspent_txouts(&self) -> bool {
        //@todo(vy): For all txins, look up txin->txid in our UTXO cache,
        // if found and pubkey matches, then txin is valid.
        return true;
    }

    fn amount_transfer_is_valid(&self, db: &Database) -> bool {
        let txouts = self.get_txouts();
        let txins = self.get_txins();
        let mut input_money = 0;

        for txin in txins.iter() {
            let mut key = Vec::with_capacity(database::KEY_LEN);
            key.push(Transaction::get_key_prefix());
            key.extend_from_slice(txin.get_tx_id().as_ref());

            match db.get_proto::<Transaction>(&key).expect("Could not retrieve tx") {
                Some(tx) => {
                    let txout_index = txin.get_txout_index();
                    if let Some(txout) = tx.get_txouts().get(txout_index as usize) {
                        input_money += txout.amount;
                    }
                },
                None => {},
            }
        }

        let output_money = txouts.iter().fold(0, |acc, txout| acc + txout.amount);

        input_money >= output_money
    }
}

impl Hashable for Transaction {
    fn update_context(&self, context: &mut Context) {
        let bytes = self.get_hash_context();
        context.update(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matcha_pb::{InputTransaction, OutputTransaction, OutputTransactionType};
    use hex;
    use rust_sodium::crypto::sign;
    use block::{self, BlockExt, SignedBlockExt};
    use database::{Storeable};
    use tempdir::TempDir;

    fn create_test_database() -> Database {
        let dir = TempDir::new("test_db").unwrap();

        Database::new_from_path(dir.path())
    }

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

    #[test]
    fn amount_transfer_is_valid() {
        let db = create_test_database();

        let (public_key, secret_key) = sign::gen_keypair();

        let mut block = block::create_block_template();
        let mut transaction = Transaction::new();
        let mut txout = OutputTransaction::new();

        txout.set_transaction_type(OutputTransactionType::NORMAL_TX);
        txout.set_amount(500);

        &transaction.txouts.push(txout);

        let txid = transaction.to_hash();

        block.mut_transactions().push(transaction);
        block.set_previous_hash(vec![0; 32]);
        block.set_transaction_root(vec![0; 32]);
        block.set_public_key_from_struct(public_key);

        let signed_block = block.sign(&secret_key).unwrap();
        let full_block = signed_block.hash();

        full_block.insert(&db).expect("Could not insert block into database");

        let mut tx_to_validate = Transaction::new();
        let mut txin_to_validate = InputTransaction::new();
        let mut txout_to_validate = OutputTransaction::new();

        txout_to_validate.set_transaction_type(OutputTransactionType::NORMAL_TX);
        txout_to_validate.set_amount(500);

        txin_to_validate.set_tx_id(txid.as_ref().to_vec());
        txin_to_validate.set_txout_index(0);

        &tx_to_validate.txins.push(txin_to_validate);
        &tx_to_validate.txouts.push(txout_to_validate);

        assert_eq!(tx_to_validate.amount_transfer_is_valid(&db), true);
    }
}
