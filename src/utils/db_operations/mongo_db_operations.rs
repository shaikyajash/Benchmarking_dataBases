use futures::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

use crate::store::user_struct::User;

//MongoDb Operations
pub trait MongoOperations {
    fn users_collection(&self) -> Collection<User>;
    async fn insert_user(&self, user: &User) -> Result<(), mongodb::error::Error>;
    async fn read_users(&self) -> Result<Vec<User>, mongodb::error::Error>;
    async fn clear_users(&self) -> Result<(), mongodb::error::Error>;
}

impl MongoOperations for Database {
    fn users_collection(&self) -> Collection<User> {
        self.collection("users")
    }
    async fn insert_user(&self, user: &User) -> Result<(), mongodb::error::Error> {
        self.users_collection().insert_one(user).await?;
        Ok(())
    }
    async fn read_users(&self) -> Result<Vec<User>, mongodb::error::Error> {
        let cursor = self.users_collection().find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }

    async fn clear_users(&self) -> Result<(), mongodb::error::Error> {
        self.users_collection().delete_many(doc! {}).await?;
        Ok(())
    }
}
