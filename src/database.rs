use rocksdb::{Error, DB, DBVector};

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
        self.connection.put(key, value)
    }

    fn get(&self, key: &[u8]) -> Result<Option<DBVector>, Error> {
        self.connection.get(key)
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
