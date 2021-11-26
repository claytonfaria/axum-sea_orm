use crate::{
    auth::jwt::Claims,
    entity::{prelude::Users, users},
    error::{ApiError, Error},
};

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;
use serde_json::{json, Value};

use super::dto::{CreateUser, UpdateUser};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route("/:id", get(get_user).delete(delete_user).put(update_user))
}

#[derive(Deserialize, Debug)]
struct Pagination {
    page: usize,
    per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 30,
        }
    }
}

async fn get_all_users(
    Extension(conn): Extension<DatabaseConnection>,
    authenticated_user: Claims,
    pagination: Option<Query<Pagination>>,
) -> Result<(StatusCode, Json<Vec<users::Model>>), ApiError> {
    let Query(pagination) = pagination.unwrap_or_default();
    dbg!(&pagination);
    dbg!(&authenticated_user);
    let users = Users::find().all(&conn).await.map_err(Error::DbError)?;

    Ok((StatusCode::OK, Json(users)))
}

async fn get_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
    // Explicit specifiying the response types
) -> Result<(StatusCode, Json<Option<users::Model>>), ApiError> {
    let found_user = Users::find_by_id(id)
        .one(&conn)
        .await
        .map_err(Error::DbError)?;

    match found_user {
        Some(user) => Ok((StatusCode::OK, Json(Some(user)))),
        None => Err(Error::NotFound)?,
    }
}

async fn create_user(
    Extension(conn): Extension<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<Option<users::Model>>), ApiError> {
    let user = users::ActiveModel {
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        email: Set(payload.email),
        gender: Set(payload.gender),
        age: Set(payload.age),
        ..Default::default()
    };

    let response = users::Entity::insert(user)
        .exec(&conn)
        .await
        .map_err(Error::DbError)?;

    let inserted_id = response.last_insert_id;

    tracing::debug!("Inserted user with id: {}", inserted_id);

    let created_user = Users::find_by_id(inserted_id)
        .one(&conn)
        .await
        .map_err(Error::DbError)?;

    if created_user.is_none() {
        tracing::error!("Failed to create user");
        return Err(Error::FailedCreateUser)?;
    }

    Ok((StatusCode::CREATED, Json(created_user)))
}

async fn delete_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let response = Users::find_by_id(id)
        .one(&conn)
        .await
        .map_err(Error::DbError)?;

    if response.is_none() {
        return Err(Error::NotFound)?;
    }

    let user: users::ActiveModel = response.unwrap().into();
    user.delete(&conn).await.map_err(Error::DbError)?;

    Ok((StatusCode::OK, Json(json!({ "message": "User deleted" }))))
}

async fn update_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUser>,
) -> Result<(StatusCode, Json<Option<users::Model>>), ApiError> {
    let response = Users::find_by_id(id)
        .one(&conn)
        .await
        .map_err(Error::DbError)?;

    if response.is_none() {
        return Err(Error::NotFound)?;
    }

    let mut user: users::ActiveModel = response.unwrap().into();

    if payload.first_name.is_some() {
        user.first_name = Set(payload.first_name.unwrap());
    }

    if payload.last_name.is_some() {
        user.last_name = Set(payload.last_name.unwrap());
    }

    if payload.email.is_some() {
        user.email = Set(payload.email);
    }

    if payload.gender.is_some() {
        user.gender = Set(payload.gender.unwrap())
    }

    if payload.age.is_some() {
        user.age = Set(payload.age);
    }

    user.update(&conn).await.map_err(Error::DbError)?;

    let updated_user = Users::find_by_id(id)
        .one(&conn)
        .await
        .map_err(Error::DbError)?;

    Ok((StatusCode::OK, Json(updated_user)))
}
