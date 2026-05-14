use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Db {
    pub pool: Pool<Postgres>,
}

pub struct UserRow {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl Db {
    pub async fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("None .env file DATABASE_URL");

        let pool = PgPoolOptions::new()
            .max_connections(20)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        Self { pool }
    }

    pub async fn create_user(&self, username: &str, hash: &str, email: &str, role: &str) -> anyhow::Result<i32> {
        let res = sqlx::query!(
            "INSERT INTO users (username, password_hash, email, role_user) VALUES ($1, $2, $3, $4) RETURNING id",
            username, hash, email, role
        )
        
        .fetch_one(&self.pool).await?;
        Ok(res.id)
    }

    pub async fn get_user_id_by_email(&self, email: &str) -> anyhow::Result<Option<i32>> {
        let res = sqlx::query!(
            "SELECT id FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(res.map(|row| row.id))
    }

    pub async fn get_user_name_by_id(&self, id_str: &str) -> anyhow::Result<Option<String>> {
        let id: i32 = id_str.parse()?;
        let res = sqlx::query!(
            "SELECT username FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(res.map(|row| row.username))
    }

    pub async fn get_role_name_by_id(&self, id_str: &str) -> anyhow::Result<Option<String>> {
        let id: i32 = id_str.parse()?;
        let res = sqlx::query!(
            "SELECT role_user FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(res.map(|row| row.role_user))
    }

    pub async fn get_user_hash_pasword_by_email(&self, email: &str) -> anyhow::Result<Option<String>> {
        let res = sqlx::query!(
            "SELECT password_hash FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(res.map(|row| row.password_hash))
    }

    pub async fn get_user_by_name(&self, username: &str) -> anyhow::Result<Option<UserRow>> {
        let user = sqlx::query_as!(
            UserRow,
            "SELECT id, username, password_hash FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn user_exists(&self, email: &str) -> anyhow::Result<bool> {
        let res = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
            email
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(res.exists.unwrap_or(false))
    }

    pub async fn update_password(&self, id: i32, new_hash: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE users SET password_hash = $1 WHERE id = $2",
            new_hash, id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn count_users(&self) -> anyhow::Result<i64> {
        let res = sqlx::query!("SELECT count(*) FROM users")
            .fetch_one(&self.pool)
            .await?;
        Ok(res.count.unwrap_or(0))
    }
}