use crate::{
    models::{RoleRepository, RoleRequest},
    ResponseError,
};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(get_all))
        .route("/", web::post().to(create))
        .route("/{id}", web::get().to(get_by_id))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete));
}

pub async fn get_all(pool: web::Data<PgPool>) -> HttpResponse {
    let repo = RoleRepository::new();
    let roles = repo.get_all(&pool).await;
    match roles {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn create(pool: web::Data<PgPool>, req: web::Json<RoleRequest>) -> HttpResponse {
    let repo = RoleRepository::new();
    let role = repo.create(&pool, req.into_inner()).await;
    match role {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn get_by_id(pool: web::Data<PgPool>, path: web::Path<i32>) -> HttpResponse {
    let repo = RoleRepository::new();
    let role = repo.get_by_id(&pool, path.into_inner()).await;
    match role {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(_) => HttpResponse::NotFound().json(ResponseError::new("Role not found".to_string())),
    }
}

pub async fn update(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: web::Json<RoleRequest>,
) -> HttpResponse {
    let repo = RoleRepository::new();
    let role = repo
        .update(&pool, path.into_inner(), req.into_inner())
        .await;
    match role {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn delete(pool: web::Data<PgPool>, path: web::Path<i32>) -> HttpResponse {
    let repo = RoleRepository::new();
    let role = repo.delete(&pool, path.into_inner()).await;
    match role {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
