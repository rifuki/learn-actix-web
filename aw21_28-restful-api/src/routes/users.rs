use actix_web::{
    web, 
    HttpResponse, 
    ResponseError, 
    http::StatusCode
};
use sqlx::{
    self,
    PgPool,
    Row
};
use argon2::{
    password_hash::{
        PasswordHash,
        PasswordVerifier
    },
    Argon2
};
use validator::Validate;

use crate::schemas;
use crate::errors::Error as AppError;
use crate::utils::{
    validation_errors_response, 
    hash_password
};

/// Register a new User
/// 
/// Please wrap the payload with `user` key
#[utoipa::path(
    post,
    path = "/api/v1/users/register",
    tag = "users",
    responses(
        (status = 201, description = "Success", body = UserRegister),
        (status = 400, description = "Bad Request")
    ),
    request_body = UserRegister
)]
pub async fn register(
    (
        form, 
        pool
    )
        : 
    (
        web::Json<schemas::In<schemas::UserRegister>>,
        web::Data<PgPool>
    )
) -> Result<HttpResponse, AppError> {
    let register_user = form.into_inner().user;

    /* * validate user input */
    let validation_result = register_user.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(
            validation_errors_response(&validation_errors)
        );
    }

    let password_hash = hash_password(
        register_user.password.as_bytes()
    )?;

    let pool = pool.get_ref();

    let query = sqlx::query("INSERT INTO users (email, username, password) VALUES ($1, $2, $3)")
        .bind(&register_user.email)
        .bind(&register_user.username)
        .bind(password_hash);


    match query.execute(pool).await {
        Ok(_) => {
            let success_response = serde_json::json!({
                "message": "Record created successfully"
            });

            Ok(
                HttpResponse::Ok().status(StatusCode::CREATED).json(success_response)
            )
        },
        Err(err) => {

            let custom_error: AppError = err.into();
            Ok(
                custom_error.error_response()
                // HttpResponse::Ok().finish()
            )
        }
    }

}


/// Login
/// 
/// Please wrap the payload with `user` key
#[utoipa::path(
    post,
    path = "/api/v1/users/login",
    tag = "users",
    responses(
        (status = 200, description = "Success", body = UserLgin),
        (status = 400, description = "Bad Request")
    ),
    request_body = UserLogin
)]
pub async fn login(
    (
        form, 
        pool
    )
        : 
    (
        web::Json<schemas::In<schemas::UserLogin>>,
        web::Data<PgPool>
    )
) -> Result<HttpResponse, AppError> {
    let login_user = form.into_inner().user;
    
    /* * validate user input */
    let validation_result = login_user.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(
            validation_errors_response(&validation_errors)
        );
    }

    let pool = pool.get_ref();

    let query = sqlx::query("SELECT password FROM users WHERE email = $1")
        .bind(&login_user.email);

    let result = query.fetch_one(pool).await;

    match result {
        Ok(row) => {
            let stored_password_hash: String = row.get("password");
            let parsed_hash = PasswordHash::new(&stored_password_hash);

            /* * verify password */
            if Argon2::default()
                .verify_password(login_user.password.as_bytes(), &parsed_hash.unwrap())
                .is_ok()
            {
                let success_response = serde_json::json!({
                    "message": "Authentication successful"
                });

                Ok(
                    HttpResponse::Ok()
                        .json(success_response)
                )
            } else {
                let error_response = serde_json::json!({
                    "message": "Authentication failed"
                });

                Ok(
                    HttpResponse::Unauthorized()
                        .json(error_response)
                )
            }
        }

        Err(err) => {
            let custom_error: AppError = err.into();
            Ok(
                custom_error.error_response()
            )
        }
    }
}


/// Update
/// 
/// Please wrap the payload with `user` key
#[utoipa::path(
    put,
    path = "/api/v1/users/update",
    tag = "users",
    responses(
        (status = 200, description = "Success", body = UserUpdate),
        (status = 400, description = "Bad Request")
    ),
    request_body = UserUpdate
)]
pub async fn update(
    (
        form, 
        pool
    )
        : 
    (
        web::Json<schemas::In<schemas::UserUpdate>>,
        web::Data<PgPool>
    )
) -> Result<HttpResponse, AppError> {
    let update_user = form.into_inner().user;
    
    /* * validate user input */
    let validation_result = update_user.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(
            validation_errors_response(&validation_errors)
        );
    }

    let hashed_password = hash_password(
        update_user.password.unwrap().as_bytes()
    )?;

    let pool = pool.get_ref();

    let query = sqlx::query("UPDATE users SET email = $1, password = $2, bio = $3 WHERE username = $4")
        .bind(&update_user.email)
        .bind(hashed_password.to_string())
        .bind(&update_user.bio)
        .bind(&update_user.username);

    match query.execute(pool).await {
        Ok(_) => {
            let success_response = serde_json::json!({
                "message": "Record updated successfully"
            });

            Ok(
                HttpResponse::Ok()
                    .json(success_response)
            )
        }
        Err(err) => {
            let custom_error: AppError = err.into();
            Ok(
                custom_error.error_response()
            )
        }
    }
}


/// Delete a User
/// 
/// Please wrap the payload with `user` key
#[utoipa::path(
    delete,
    path = "/api/v1/users/delete",
    tag = "users",
    responses(
        (status = 200, description = "Success", body = UserDelete),
        (status = 400, description = "Bad Request")
    ),
    request_body = UserDelete
)]
pub async fn delete(
    (
        form, 
        pool
    )
        : 
    (
        web::Json<schemas::In<schemas::UserDelete>>,
        web::Data<PgPool>
    )
) -> Result<HttpResponse, AppError> {
    let user_info = form.into_inner().user;
    
    /* * validate user input */
    let validation_result = user_info.validate();
    if let Err(validation_errors) = validation_result {
        return Ok(
            validation_errors_response(&validation_errors)
        );
    }

    let pool = pool.get_ref();

    let query = sqlx::query("DELETE FROM users WHERE username = $1")
        .bind(&user_info.username);

    let result = query.execute(pool).await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                let success_response = serde_json::json!({
                    "message": "Record deleted successfully"
                });

                Ok(
                    HttpResponse::Ok()
                        .json(success_response)
                )
            } else {
                let error_response = serde_json::json!({
                    "message": "Record not found for the provided username"
                });

                Ok(
                    HttpResponse::NotFound()
                        .json(error_response)
                )
            }
        }

        Err(err) => {
            let custom_error: AppError = err.into();
            Ok(
                custom_error.error_response()
            )
        }
    }
}