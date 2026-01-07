use rusty_leveldb::{DB as LevelDB, LdbIterator};
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

impl LevelOperations for Arc<Mutex<LevelDB>> {
    fn insert_user(&self, user: &User) -> Result<(), LevelError> {
        let key = format!("user:{}", user.id);
        let value = match serde_json::to_vec(user) {
            Ok(v) => v,
            Err(e) => return Err(LevelError::Serialization(e.to_string())),
        };

        let mut db = match self.lock() {
            Ok(db) => db,
            Err(e) => return Err(LevelError::Lock(e.to_string())),
        };

        match db.put(key.as_bytes(), &value) {
            Ok(_) => Ok(()),
            Err(e) => Err(LevelError::Db(format!("{:?}", e))),
        }
    }
    fn read_users(&self) -> Result<Vec<User>, LevelError> {
        let mut users = Vec::new();
        let mut db = match self.lock() {
            Ok(db) => db,
            Err(e) => return Err(LevelError::Lock(e.to_string())),
        };

        let mut iter = match db.new_iter() {
            Ok(iter) => iter,
            Err(e) => return Err(LevelError::Db(format!("{:?}", e))),
        };

        iter.seek(b"user:");

        let mut key = Vec::new();
        let mut value = Vec::new();

        while iter.valid() {
            if !iter.current(&mut key, &mut value) {
                break;
            }

            if !key.starts_with(b"user:") {
                break;
            }

            let user: User = match serde_json::from_slice(&value) {
                Ok(u) => u,
                Err(e) => return Err(LevelError::Serialization(e.to_string())),
            };

            users.push(user);
            iter.advance();
        }

        Ok(users)
    }

    fn clear_users(&self) -> Result<(), LevelError> {
        let mut db = match self.lock() {
            Ok(db) => db,
            Err(e) => return Err(LevelError::Lock(e.to_string())),
        };

        let mut iter = match db.new_iter() {
            Ok(iter) => iter,
            Err(e) => return Err(LevelError::Db(format!("{:?}", e))),
        };

        iter.seek(b"user:");

        let mut keys_to_delete: Vec<Vec<u8>> = Vec::new();
        let mut key = Vec::new();
        let mut value = Vec::new();

        while iter.valid() {
            if !iter.current(&mut key, &mut value) {
                break;
            }

            if !key.starts_with(b"user:") {
                break;
            }

            keys_to_delete.push(key.clone());
            iter.advance();
        }

        drop(iter);

        for k in keys_to_delete {
            match db.delete(&k) {
                Ok(_) => {}
                Err(e) => return Err(LevelError::Db(format!("{:?}", e))),
            }
        }

        Ok(())
    }
}
