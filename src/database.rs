use std::path::Path;

use rust_sodium::crypto::hash;
use rocksdb::{DB, DBVector};
use failure::Error;

use protobuf::{self, MessageStatic};
use matcha_pb::{Transaction, FullBlock};
use transaction::TransactionExt;
use config;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Database {
    connection: DB
}

pub const KEY_LEN: usize = hash::sha256::DIGESTBYTES + 1;

#[allow(dead_code)]
impl Database {
    pub fn new() -> Database {
        let database_path = config::get_config_dir().join("db");
        let db = DB::open_default(database_path).unwrap();

        Database {
            connection: db
        }
    }

    pub fn new_from_path(path: &Path) -> Database {
        let db = DB::open_default(path).unwrap();

        Database {
            connection: db
        }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.connection.put(key, value).map_err(|e| { format_err!("An error occured: {}", e) })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<DBVector>, Error> {
        self.connection.get(key).map_err(|e| { format_err!("An error occured: {}", e) })
    }

    pub fn put_proto<T: MessageStatic + Storeable>(&self, msg: &T) -> Result<(), Error> {
        let key = msg.get_key();

        match msg.write_to_bytes() {
            Ok(buf) => {
                match self.put(key.as_slice(), buf.as_slice()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format_err!("An error occured: {}", e)),
                }
            },
            Err(e) => Err(format_err!("An error occured: {}", e)),
        }
    }

    pub fn get_proto<T: MessageStatic>(&self, key: &[u8]) -> Result<Option<T>, Error> {
        match self.get(key) {
            Ok(Some(value)) => {
                match protobuf::parse_from_bytes(value.as_ref()) {
                    Ok(msg) => Ok(Some(msg)),
                    Err(e) => Err(format_err!("An error occured: {}", e)),
                }
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format_err!("An error occured: {}", e)),
        }
    }
}

pub trait Storeable {
    fn get_key_prefix() -> u8;
    fn get_key(&self) -> Vec<u8>;
    fn insert(&self, database: &Database) -> Result<(), Error>;
}

impl Storeable for FullBlock {
    fn get_key_prefix() -> u8 {
        'b' as u8
    }

    fn get_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(KEY_LEN);
        let hash = self.get_hash();

        key.push(FullBlock::get_key_prefix());
        key.extend_from_slice(hash.as_ref());

        key
    }

    fn insert(&self, database: &Database) -> Result<(), Error> {
        let transactions = self.get_signed_block().get_block().get_transactions();

        transactions.iter().for_each(move |transaction| {
            transaction.insert(&database).expect("Could not insert transaction")
        });

        database.put_proto(self)
    }
}

impl Storeable for Transaction {
    fn get_key_prefix() -> u8 {
        't' as u8
    }

    fn get_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(KEY_LEN);
        let hash = self.to_hash();

        key.push(Transaction::get_key_prefix());
        key.extend_from_slice(hash.as_ref());

        key
    }

    fn insert(&self, database: &Database) -> Result<(), Error> {
        database.put_proto(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use rust_sodium::crypto::sign;

    use matcha_pb::{OutputTransaction, OutputTransactionType, Transaction};
    use block::{self, BlockExt, SignedBlockExt};

    fn create_test_database() -> Database {
        let dir = TempDir::new("test_db").unwrap();

        Database::new_from_path(dir.path())
    }

    #[test]
    fn can_create_database() {
        let db = create_test_database();

        db.put(b"hello", b"world").expect("Could not set key-value");

        let result =
            match db.get(b"hello") {
                Ok(Some(value)) => value.to_vec(),
                Ok(None) => vec![0; 1],
                Err(_) => vec![0; 1],
            };

        assert_eq!(result, b"world");
    }

    #[test]
    fn can_insert_and_retrieve_full_block() {
        let db = create_test_database();

        let mut full_block = FullBlock::new();
        full_block.set_hash(vec![1; 32]);

        db.put_proto(&full_block).expect("Could not insert full block into database");

        let full_block_key = full_block.get_key();
        let db_full_block: FullBlock = db.get_proto(full_block_key.as_ref())
            .expect("Could not retrieve full block")
            .unwrap();

        assert_eq!(full_block.get_hash(), db_full_block.get_hash());
    }

    #[test]
    fn can_insert_and_retrieve_transaction() {
        let db = create_test_database();

        let mut transaction = Transaction::new();
        let mut txout = OutputTransaction::new();

        txout.set_transaction_type(OutputTransactionType::NORMAL_TX);
        txout.set_amount(500);

        transaction.mut_txouts().push(txout);

        db.put_proto(&transaction).expect("Could not insert transaction into database");

        let transaction_key = transaction.get_key();
        let db_transaction: Transaction = db.get_proto(transaction_key.as_ref())
            .expect("Could not retrieve transaction")
            .unwrap();

        assert_eq!(db_transaction.get_txouts().len(), 1);
    }

    #[test]
    fn inserting_blocks_inserts_their_transactions() {
        let db = create_test_database();
        let (public_key, secret_key) = sign::gen_keypair();

        let mut block = block::create_block_template();
        let mut transaction = Transaction::new();
        let mut txout = OutputTransaction::new();

        txout.set_transaction_type(OutputTransactionType::NORMAL_TX);
        txout.set_amount(500);

        transaction.mut_txouts().push(txout);

        let transaction_key = transaction.get_key();

        block.mut_transactions().push(transaction);
        block.set_previous_hash(vec![0; 32]);
        block.set_transaction_root(vec![0; 32]);
        block.set_public_key_from_struct(public_key);

        let signed_block = block.sign(&secret_key).unwrap();
        let full_block = signed_block.hash();
        let full_block_key = full_block.get_key();

        full_block.insert(&db).expect("Could not insert block into database");

        let db_transaction: Transaction = db.get_proto(transaction_key.as_ref())
            .expect("Could not retrieve transaction")
            .unwrap();

        let db_full_block: FullBlock = db.get_proto(full_block_key.as_ref())
            .expect("Could not retrieve full block")
            .unwrap();

        assert_eq!(db_transaction.get_txouts().len(), 1);
        assert_eq!(full_block.get_hash(), db_full_block.get_hash());
    }
}
