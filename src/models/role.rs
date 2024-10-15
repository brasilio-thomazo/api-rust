use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RoleRequest {
    name: String,
}

pub struct RoleRepository {}

impl RoleRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, pool: &PgPool, req: RoleRequest) -> Result<Role, Error> {
        let sql = "INSERT INTO roles (name) VALUES ($1) RETURNING *";
        sqlx::query_as::<_, Role>(sql)
            .bind(req.name)
            .fetch_one(pool)
            .await
    }

    pub async fn get_all(&self, pool: &PgPool) -> Result<Vec<Role>, Error> {
        let sql = "SELECT * FROM roles";
        sqlx::query_as::<_, Role>(sql).fetch_all(pool).await
    }

    pub async fn get_by_id(&self, pool: &PgPool, id: i32) -> Result<Role, Error> {
        let sql = "SELECT * FROM roles WHERE id = $1";
        sqlx::query_as::<_, Role>(sql)
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn update(&self, pool: &PgPool, id: i32, req: RoleRequest) -> Result<Role, Error> {
        let sql = "UPDATE roles SET name = $1 WHERE id = $2 RETURNING *";
        sqlx::query_as::<_, Role>(sql)
            .bind(req.name)
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(&self, pool: &PgPool, id: i32) -> Result<Role, Error> {
        let sql = "DELETE FROM roles WHERE id = $1 RETURNING *";
        sqlx::query_as::<_, Role>(sql)
            .bind(id)
            .fetch_one(pool)
            .await
    }
}
