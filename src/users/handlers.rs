use super::models::User;
use crate::db;
use warp::http::StatusCode;
use std::convert::Infallible;
use warp::reply::Json;
use serde_json::json;
use argon2::{self, Config};
use rand::Rng;

pub async fn create_user(new_user: User) -> Result<impl warp::Reply, Infallible> {
    match db::connect_db().await {
        Ok(client) => {
            // 비밀번호 해싱
            let salt: [u8; 32] = rand::thread_rng().gen();
            let config = Config::default();
            let hash = argon2::hash_encoded(new_user.password.as_bytes(), &salt, &config).unwrap();

            let result = client.execute(
                "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
                &[&new_user.name, &new_user.email, &hash],
            ).await;

            match result {
                Ok(_) => Ok(warp::reply::with_status("User created", StatusCode::CREATED)),
                Err(e) => {
                    eprintln!("Error creating user: {}", e);
                    Ok(warp::reply::with_status(
                        "Failed to create user",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        },
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            Ok(warp::reply::with_status(
                "Database connection error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
