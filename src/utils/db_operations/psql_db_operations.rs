use sqlx::PgPool;

use crate::store::user_struct::User;

pub trait PgOperations {
    async fn read_users(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn insert_users(&self, user: &User) -> Result<(), sqlx::Error>;
    async fn clear_users(&self) -> Result<(), sqlx::Error>;
}

impl PgOperations for PgPool {
    async fn insert_users(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, age, active)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.age)
        .bind(&user.active)
        .execute(self)
        .await?;

        Ok(())
    }

    async fn read_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, age, active
            FROM users
        "#,
        )
        .fetch_all(self)
        .await?;
        Ok(users)
    }

    async fn clear_users(&self) -> Result<(), sqlx::Error> {
        sqlx::query("TRUNCATE TABLE users").execute(self).await?;
        Ok(())
    }
}
