use std::time::{SystemTime, UNIX_EPOCH};

use rust_sodium::crypto::sign::{self, PublicKey, SecretKey, Signature};
use rust_sodium::crypto::sign::{SIGNATUREBYTES};
use rust_sodium::crypto::hash::sha256;
use ring::digest::SHA256;
use bytes::{BytesMut, BufMut, LittleEndian};
use failure::Error;
use merkle::MerkleTree;

use matcha_pb::{Block, SignedBlock, FullBlock, Transaction};
use constants;
use transaction::TransactionExt;
use database::Database;

/*
 * Hashing:
 *
 * 1) Serialize structs as protobuf messages, and we hash the full serialized payload
 * 2) Grab all the fields, put them in a bytebuffer, and hash that <-
 *
 * Storage (RocksDB k/v store):
 *
 * 1) Store the protobuf messages on disk <-
 * 2) ~Store in our format~
 *
 * RocksDB layout:
 *   b<FullBlock.hash> -> FullBlock
 *   t<transaction.hash> -> Transaction
 *   u<UnspentTxout> -> TXOUT
 *
 * b: FullBlock
 * t: Transaction
 */

const BLOCK_HEADER_SIZE: usize = 108;

#[allow(dead_code)]
const FULL_BLOCK_HEADER_SIZE: usize = BLOCK_HEADER_SIZE + SIGNATUREBYTES;

pub trait BlockExt {
    fn set_public_key_from_struct(&mut self, public_key: PublicKey);
    fn get_header(&self) -> BytesMut;
    fn sign(&self, secret_key: &SecretKey) -> Result<SignedBlock, Error>;
    fn valid_header(&self) -> bool;
    fn get_merkle_tree(&self) -> MerkleTree<Transaction>;
    fn get_merkle_root(&self) -> Vec<u8>;
}

impl BlockExt for Block {
    fn set_public_key_from_struct(&mut self, public_key: PublicKey) {
        self.set_public_key(public_key.0.to_vec());
    }

    fn get_header(&self) -> BytesMut {
        let mut buffer = BytesMut::with_capacity(BLOCK_HEADER_SIZE);

        buffer.put_u32::<LittleEndian>(self.get_version());
        buffer.put_slice(self.get_public_key());
        buffer.put_slice(self.get_previous_hash());
        buffer.put_slice(self.get_transaction_root());
        buffer.put_u64::<LittleEndian>(self.get_timestamp());
        buffer
    }

    fn sign(&self, secret_key: &SecretKey) -> Result<SignedBlock, Error> {
        if self.valid_header() {
            let mut signed_block = SignedBlock::new();
            let mut header = self.get_header();
            let signature = sign::sign_detached(&mut header.take(), secret_key);

            signed_block.set_signature(signature.0.to_vec());
            signed_block.set_block(self.clone());

            Ok(signed_block)
        } else {
            Err(format_err!("Block was invalid"))
        }
    }

    fn valid_header(&self) -> bool {
        let mut number_of_bytes = 0;

        number_of_bytes += 4;
        number_of_bytes += self.get_public_key().len();
        number_of_bytes += self.get_previous_hash().len();
        number_of_bytes += self.get_transaction_root().len();
        number_of_bytes += 8;

        return number_of_bytes == BLOCK_HEADER_SIZE;
    }

    fn get_merkle_tree(&self) -> MerkleTree<Transaction> {
        MerkleTree::from_vec(&SHA256, self.get_transactions().to_vec())
    }

    fn get_merkle_root(&self) -> Vec<u8> {
        let tree = self.get_merkle_tree();
        tree.root_hash().clone()
    }
}

pub trait SignedBlockExt {
    fn get_signature_struct(&self) -> Result<Signature, Error>;
    fn verify(&self, public_key: &PublicKey) -> bool;
    fn get_header(&self) -> BytesMut;
    fn compute_hash(&self) -> sha256::Digest;
    fn hash(&self) -> FullBlock;
}

impl SignedBlockExt for SignedBlock {
    fn get_signature_struct(&self) -> Result<Signature, Error> {
        let signature_bytes = self.get_signature();

        match Signature::from_slice(signature_bytes) {
            Some(signature) => Ok(signature),
            None => Err(format_err!("Invalid signature")),
        }
    }

    fn verify(&self, public_key: &PublicKey) -> bool {
        let signature = self.get_signature_struct().expect("Invalid signature");
        let block = self.get_block();

        if block.valid_header() && self.get_signature().len() == SIGNATUREBYTES {
            let mut header = block.get_header();
            sign::verify_detached(&signature, &mut header.take(), public_key)
        } else {
            false
        }
    }

    fn get_header(&self) -> BytesMut {
        let mut header = self.get_block().get_header();
        header.extend_from_slice(self.get_signature());
        header
    }

    fn compute_hash(&self) -> sha256::Digest {
        let header = self.get_header();
        sha256::hash(&header)
    }

    fn hash(&self) -> FullBlock {
        let mut full_block = FullBlock::new();
        let digest = self.compute_hash();

        full_block.set_signed_block(self.clone());
        full_block.set_hash(digest.0.to_vec());
        full_block
    }
}

pub trait FullBlockExt {
    fn is_valid(&self, db: &Database) -> bool;
    fn witness_signature_is_valid(&self) -> bool;
    fn transactions_are_valid(&self, db: &Database) -> bool;
    fn transaction_root_is_valid(&self) -> bool;
    fn block_hash_is_valid(&self) -> bool;
}

impl FullBlockExt for FullBlock {
    /*
     * A FullBlock is valid if all these conditions are true:
     * - Signed by a valid witness in their respective production timeslot.
     * - All transactions are valid
     * - Transaction root hash is valid
     * - Hash of the block is valid
     * - First transaction is a coinbase transaction, which contains reward, left over fees, and maintenance fees
     */

    fn is_valid(&self, db: &Database) -> bool {
        return self.witness_signature_is_valid() &&
            self.transactions_are_valid(db) &&
            self.transaction_root_is_valid() &&
            self.block_hash_is_valid();
    }

    fn witness_signature_is_valid(&self) -> bool {
        // @TODO(vy): Add in real witness validation here, by taking into account all voted
        // delegates.
        true
    }

    fn transactions_are_valid(&self, db: &Database) -> bool {
        let block = self.get_signed_block().get_block();
        let transactions = block.get_transactions();

        transactions.into_iter().all(|transaction| {
            transaction.is_valid(&db)
        })
    }

    fn transaction_root_is_valid(&self) -> bool {
        // @TODO(vy): Create a merkle tree using transaction hashes to verify transaction root
        // hash.
        true
    }

    fn block_hash_is_valid(&self) -> bool {
        let signed_block = self.get_signed_block();
        let block_hash = self.get_hash();
        let digest = signed_block.compute_hash();

        block_hash == digest.0
    }
}

#[allow(dead_code)]
pub fn create_block_template() -> Block {
    let mut block = Block::new();
    let current_timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time is behind UNIX_EPOCH")
        .as_secs();

    block.set_version(constants::MATCHA_VERSION);
    block.set_timestamp(current_timestamp);
    block
}

/*
 * As a miner/witness:
 * - Create a block template
 * - Add our witness pubkey to the block
 * - Add valid transactions into the block from our transaction mempool (can't do yet)
 * - Compute leftovers as reward for self (last)
 * - Sign the block (Block -> SignedBlock)
 * - Hash the block (SignedBlock -> FullBlock)
 */

#[cfg(test)]
mod tests {
    use super::*;
    use matcha_pb::Transaction;
    use tempdir::TempDir;

    fn create_test_database() -> Database {
        let dir = TempDir::new("test_db").unwrap();

        Database::new_from_path(dir.path())
    }

    #[test]
    fn can_create_block_template() {
        let block = create_block_template();

        assert_eq!(block.get_version(), constants::MATCHA_VERSION);
    }

    #[test]
    fn can_validate_block_header() {
        let mut block = create_block_template();
        let (public_key, _) = sign::gen_keypair();

        block.set_previous_hash(vec![0; 32]);
        block.set_transaction_root(vec![0; 32]);
        block.set_public_key_from_struct(public_key);

        assert_eq!(block.valid_header(), true);
    }

    #[test]
    fn can_add_pubkey_to_block_and_sign() {
        let mut block = create_block_template();
        let (public_key, secret_key) = sign::gen_keypair();

        block.set_previous_hash(vec![0; 32]);
        block.set_transaction_root(vec![0; 32]);
        block.set_public_key_from_struct(public_key);

        let signed_block = block.sign(&secret_key).unwrap();

        assert_eq!(signed_block.verify(&public_key), true);
    }

    #[test]
    fn can_hash_signed_block() {
        let mut block = create_block_template();
        let (public_key, secret_key) = sign::gen_keypair();

        block.set_previous_hash(vec![0; 32]);
        block.set_transaction_root(vec![0; 32]);
        block.set_public_key_from_struct(public_key);

        let signed_block = block.sign(&secret_key).unwrap();
        let full_block = signed_block.hash();
        let header = signed_block.get_header();
        let digest = sha256::hash(&header);

        assert_eq!(header.len(), FULL_BLOCK_HEADER_SIZE);
        assert_eq!(full_block.get_hash(), digest.0);
    }

    #[test]
    fn can_validate_full_blocks() {
        let db = create_test_database();
        let mut block = create_block_template();
        let (public_key, secret_key) = sign::gen_keypair();

        block.set_previous_hash(vec![0; 32]);
        block.set_transaction_root(vec![0; 32]);
        block.set_public_key_from_struct(public_key);

        let signed_block = block.sign(&secret_key).unwrap();
        let full_block = signed_block.hash();

        assert_eq!(full_block.is_valid(&db), true);
    }

    #[test]
    fn can_generate_merkle_root_hash_for_block() {
        let mut block = create_block_template();
        let transaction = Transaction::new();
        let copy_transaction = transaction.clone();

        block.mut_transactions().push(transaction);

        let tree = block.get_merkle_tree();
        let proof = tree.gen_proof(copy_transaction).unwrap();

        assert_eq!(
            proof.validate(tree.root_hash()),
            true
        );
    }
}
