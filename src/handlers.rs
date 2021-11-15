use crate::{
    dto::{CreateUser, UpdateUser},
    entity::{prelude::Users, users},
};

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

// basic handler that responds with a static string
pub async fn get_all_users(Extension(conn): Extension<DatabaseConnection>) -> impl IntoResponse {
    let users = Users::find().all(&conn).await;

    match users {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
    // Explicit specifiying the response types
) -> Result<(StatusCode, Json<Option<users::Model>>), (StatusCode, String)> {
    let found_user = Users::find_by_id(id)
        .one(&conn)
        .await
        .map_err(internal_error)?;

    match found_user {
        Some(user) => Ok((StatusCode::OK, Json(Some(user)))),
        None => return Err((StatusCode::NOT_FOUND, "user not found".to_string())),
    }
}

pub async fn create_user(
    Extension(conn): Extension<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
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
        .map_err(internal_error)?;

    let inserted_id = response.last_insert_id;

    tracing::debug!("Inserted user with id: {}", inserted_id);

    let created_user = Users::find_by_id(inserted_id)
        .one(&conn)
        .await
        .map_err(internal_error)?;

    match created_user {
        Some(user) => Ok((StatusCode::OK, Json(Some(user)))),
        None => return Err((StatusCode::NOT_FOUND, "user not found".to_string())),
    }
}

pub async fn delete_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let response = Users::find_by_id(id)
        .one(&conn)
        .await
        .map_err(internal_error)?;

    match response {
        Some(user) => {
            let user: users::ActiveModel = user.into();
            user.delete(&conn).await.map_err(internal_error)?;

            Ok((StatusCode::OK, "User deleted successfully"))
        }
        None => return Err((StatusCode::NOT_FOUND, "user not found".to_string())),
    }
}

pub async fn update_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let response = Users::find_by_id(id)
        .one(&conn)
        .await
        .map_err(internal_error)?;

    match response {
        Some(user) => {
            let mut user: users::ActiveModel = user.into();

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

            user.update(&conn).await.map_err(internal_error)?;

            let updated_user = Users::find_by_id(id)
                .one(&conn)
                .await
                .map_err(internal_error)?;

            Ok((StatusCode::OK, Json(updated_user)))
        }
        None => return Err((StatusCode::NOT_FOUND, "user not found".to_string())),
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
