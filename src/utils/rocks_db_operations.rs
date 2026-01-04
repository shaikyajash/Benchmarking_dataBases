use rocksdb::DB as RocksDB;

use crate::store::users::User;

// Custom error type for RocksDB operations
#[derive(Debug)]
pub enum RocksError {
    RocksDb(rocksdb::Error),
    Serialization(String),
}

impl From<rocksdb::Error> for RocksError {
    fn from(err: rocksdb::Error) -> Self {
        RocksError::RocksDb(err)
    }
}

impl std::fmt::Display for RocksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RocksError::RocksDb(e) => write!(f, "RocksDB error: {}", e),
            RocksError::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

//RocksDB Operations
pub trait RocksOperations {
    fn insert_user(&self, user: &User) -> Result<(), RocksError>;
    fn read_users(&self) -> Result<Vec<User>, RocksError>;
    fn clear_users(&self) -> Result<(), RocksError>;
}

impl RocksOperations for RocksDB {
    fn insert_user(&self, user: &User) -> Result<(), RocksError> {
        let key = format!("user:{}", user.id);
        let value = match serde_json::to_vec(user) {
            Ok(v) => v,
            Err(e) => {
                return Err(RocksError::Serialization(e.to_string()));
            }
        };
        self.put(key.as_bytes(), value)?;
        Ok(())
    }

    fn read_users(&self) -> Result<Vec<User>, RocksError> {
        let mut users = Vec::new();
        let iter = self.prefix_iterator(b"user:");

        for item in iter {
            let (_, value) = match item {
                Ok(kv) => kv,
                Err(e) => return Err(RocksError::RocksDb(e)),
            };

            let user: User = match serde_json::from_slice(&value) {
                Ok(u) => u,
                Err(e) => {
                    return Err(RocksError::Serialization(e.to_string()));
                }
            };
            users.push(user);
        }

        Ok(users)
    }

    fn clear_users(&self) -> Result<(), RocksError> {
        let iter = self.prefix_iterator(b"user:");

        for item in iter {
            let (key, _) = match item {
                Ok(kv) => kv,
                Err(e) => return Err(RocksError::RocksDb(e)),
            };
            self.delete(&key)?;
        }

        Ok(())
    }
}
