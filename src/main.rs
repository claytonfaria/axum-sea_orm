use self::entity::prelude::Users;
use self::entity::users;
mod entity;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    AddExtensionLayer, Json, Router,
};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_sea_orm=debug,tower_http=trace")
    }

    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    // build our application with a route
    let app = Router::new()
        .route("/users", get(get_all_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).delete(delete_user).put(update_user),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(conn));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn get_all_users(Extension(conn): Extension<DatabaseConnection>) -> impl IntoResponse {
    let users = Users::find().all(&conn).await;

    match users {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn get_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
    // Explicit specifiying the response types
) -> Result<(StatusCode, Json<Option<users::Model>>), (StatusCode, String)> {
    let found_user = Users::find_by_id(id).one(&conn).await;

    match found_user {
        Ok(user) => match user {
            Some(user) => Ok((StatusCode::OK, Json(Some(user)))),
            None => return Err((StatusCode::NOT_FOUND, "user not found".to_string())),
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
    }
}

async fn create_user(
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

    let response = users::Entity::insert(user).exec(&conn).await;

    let inserted_id = match response {
        Ok(r) => r.last_insert_id,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    tracing::debug!("Inserted user with id: {}", inserted_id);

    let created_user = Users::find_by_id(inserted_id).one(&conn).await;

    match created_user {
        Ok(user) => match user {
            Some(user) => Ok((StatusCode::OK, Json(Some(user)))),
            None => return Err((StatusCode::NOT_FOUND, "user not found".to_string())),
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn delete_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let response = Users::find_by_id(id).one(&conn).await;

    match response {
        Ok(user) => match user {
            Some(user) => {
                let user: users::ActiveModel = user.into();
                let response = user.delete(&conn).await;

                match response {
                    Ok(_) => Ok((StatusCode::OK, "User deleted successfully")),
                    Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
                }
            }
            None => return Err((StatusCode::NOT_FOUND, "user not found".to_string())),
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn update_user(
    Extension(conn): Extension<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let response = Users::find_by_id(id).one(&conn).await;

    match response {
        Ok(user) => match user {
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

                let response = user.update(&conn).await;

                match response {
                    Ok(_) => Ok((StatusCode::OK, "User updated successfully")),
                    Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
                }
            }
            None => return Err((StatusCode::NOT_FOUND, "user not found".to_string())),
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    first_name: String,
    last_name: String,
    email: Option<String>,
    gender: String,
    age: Option<i32>,
}

#[derive(Deserialize)]
struct UpdateUser {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    gender: Option<String>,
    age: Option<i32>,
}
