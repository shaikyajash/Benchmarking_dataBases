use leveldb::database::Database as LevelDB;
use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{ReadOptions, WriteOptions};
use std::sync::{Arc, Mutex};

use crate::store::user_struct::User;

// Custom error type for LevelDB operations
#[derive(Debug)]
pub enum LevelError {
    Lock(String),
    Db(String),
    Serialization(String),
}

impl std::fmt::Display for LevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelError::Lock(e) => write!(f, "Lock error: {}", e),
            LevelError::Db(e) => write!(f, "LevelDB error: {}", e),
            LevelError::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

//LevelDB Operations
pub trait LevelOperations {
    fn insert_user(&self, user: &User) -> Result<(), LevelError>;
    fn read_users(&self) -> Result<Vec<User>, LevelError>;
    fn clear_users(&self) -> Result<(), LevelError>;
}

impl LevelOperations for Arc<Mutex<LevelDB<i32>>> {
    fn insert_user(&self, user: &User) -> Result<(), LevelError> {
        let value = match serde_json::to_vec(user) {
            Ok(v) => v,
            Err(e) => return Err(LevelError::Serialization(e.to_string())),
        };

        let db = match self.lock() {
            Ok(db) => db,
            Err(e) => return Err(LevelError::Lock(e.to_string())),
        };

        // FNV-1a hash: UUID string → i32 key
        let key = user
            .id
            .as_bytes()
            .iter()
            .fold(2166136261u32, |hash, &byte| {
                (hash ^ (byte as u32)).wrapping_mul(16777619)
            }) as i32;

        let write_opts = WriteOptions::new();

        match db.put(write_opts, key, &value) {
            Ok(_) => Ok(()),
            Err(e) => Err(LevelError::Db(format!("{:?}", e))),
        }
    }

    fn read_users(&self) -> Result<Vec<User>, LevelError> {
        let mut users = Vec::new();

        let db = match self.lock() {
            Ok(db) => db,
            Err(e) => return Err(LevelError::Lock(e.to_string())),
        };

        let read_opts = ReadOptions::new();
        let iter = db.iter(read_opts);

        for (_, value) in iter {
            let user: User = match serde_json::from_slice(&value) {
                Ok(u) => u,
                Err(e) => return Err(LevelError::Serialization(e.to_string())),
            };
            users.push(user);
        }

        Ok(users)
    }

    fn clear_users(&self) -> Result<(), LevelError> {
        let db = match self.lock() {
            Ok(db) => db,
            Err(e) => return Err(LevelError::Lock(e.to_string())),
        };

        let read_opts = ReadOptions::new();
        let keys_to_delete: Vec<i32> = db.iter(read_opts).map(|(k, _)| k).collect();

        drop(db);

        let db = match self.lock() {
            Ok(db) => db,
            Err(e) => return Err(LevelError::Lock(e.to_string())),
        };

        let write_opts = WriteOptions::new();
        for key in keys_to_delete {
            match db.delete(write_opts, key) {
                Ok(_) => {}
                Err(e) => return Err(LevelError::Db(format!("{:?}", e))),
            }
        }

        Ok(())
    }
}
