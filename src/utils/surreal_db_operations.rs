use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};

use crate::store::users::User;

// SurrealDB specific user struct (handles Thing ID type)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurrealUser {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub active: bool,
}

//SurrealDB Operations
pub trait SurrealOperations {
    async fn insert_user(&self, user: &User) -> Result<(), surrealdb::Error>;
    async fn read_users(&self) -> Result<Vec<User>, surrealdb::Error>;
    async fn clear_users(&self) -> Result<(), surrealdb::Error>;
}

impl SurrealOperations for Surreal<Client> {
    async fn insert_user(&self, user: &User) -> Result<(), surrealdb::Error> {
        let _: Option<SurrealUser> = self
            .create(("users", user.id.clone()))
            .content(user.clone())
            .await?;
        Ok(())
    }

    async fn read_users(&self) -> Result<Vec<User>, surrealdb::Error> {
        let surreal_users: Vec<SurrealUser> = self.select("users").await?;

        let mut users = Vec::new();
        for su in surreal_users {
            let id_string = match su.id.id {
                surrealdb::sql::Id::String(s) => s,
                surrealdb::sql::Id::Number(n) => n.to_string(),
                _ => su.id.id.to_string(),
            };

            users.push(User {
                id: id_string,
                name: su.name,
                email: su.email,
                age: su.age,
                active: su.active,
            });
        }

        Ok(users)
    }

    async fn clear_users(&self) -> Result<(), surrealdb::Error> {
        let _: Vec<SurrealUser> = self.delete("users").await?;
        Ok(())
    }
}
