use rocksdb::{DB, DBVector};
use protobuf::{self, MessageStatic};
use failure::Error;

use config;

#[allow(dead_code)]
struct Database {
    connection: DB
}

#[allow(dead_code)]
impl Database {
    fn new() -> Database {
        let database_path = config::get_config_dir().join("db");
        let db = DB::open_default(database_path).unwrap();

        Database {
            connection: db
        }
    }

    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.connection.put(key, value).map_err(|e| { format_err!("An error occured: {}", e) })
    }

    fn get(&self, key: &[u8]) -> Result<Option<DBVector>, Error> {
        self.connection.get(key).map_err(|e| { format_err!("An error occured: {}", e) })
    }

    fn put_proto<T: MessageStatic>(&self, msg: &T, key: &[u8]) -> Result<(), Error> {
        match msg.write_to_bytes() {
            Ok(buf) => {
                match self.put(key, buf.as_slice()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format_err!("An error occured: {}", e)),
                }
            },
            Err(e) => Err(format_err!("An error occured: {}", e)),
        }
    }

    fn get_proto<T: MessageStatic>(&self, key: &[u8]) -> Result<Option<T>, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_database() {
        config::create_config_dir();

        let db = Database::new();

        db.put(b"hello", b"world").expect("Could not set key-value");

        let result =
            match db.get(b"hello") {
                Ok(Some(value)) => value.to_vec(),
                Ok(None) => vec![0; 1],
                Err(_) => vec![0; 1],
            };

        assert_eq!(result, b"world");
    }
}
