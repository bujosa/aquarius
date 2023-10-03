use crate::api::ApiTags;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use slab::Slab;
use tokio::sync::Mutex;

use super::{
    model::User,
    utils::{
        CreateUserResponse, DeleteUserResponse, FindUserResponse, UpdateUser, UpdateUserResponse,
    },
};

// This struct is used to store user data, you can replace it with your own database.
#[derive(Default)]
pub struct UserApi {
    users: Mutex<Slab<User>>,
}

// We can use the #[OpenApi] attribute to generate the OpenAPI documentation for the API,
// and Also generate the endpoint for the API.
#[OpenApi]
impl UserApi {
    /// Create a new user
    #[oai(path = "/users", method = "post", tag = "ApiTags::User")]
    async fn create_user(&self, user: Json<User>) -> CreateUserResponse {
        let mut users = self.users.lock().await;
        let id = users.insert(user.0) as i64;
        CreateUserResponse::Ok(Json(id))
    }

    /// Find user by id
    #[oai(path = "/users/:user_id", method = "get", tag = "ApiTags::User")]
    async fn find_user(&self, user_id: Path<i64>) -> FindUserResponse {
        let users = self.users.lock().await;
        match users.get(user_id.0 as usize) {
            Some(user) => FindUserResponse::Ok(Json(user.clone())),
            None => FindUserResponse::NotFound,
        }
    }

    /// Delete user by id
    #[oai(path = "/users/:user_id", method = "delete", tag = "ApiTags::User")]
    async fn delete_user(&self, user_id: Path<i64>) -> DeleteUserResponse {
        let mut users = self.users.lock().await;
        let user_id = user_id.0 as usize;
        if users.contains(user_id) {
            users.remove(user_id);
            DeleteUserResponse::Ok
        } else {
            DeleteUserResponse::NotFound
        }
    }

    /// Update user by id
    #[oai(path = "/users/:user_id", method = "put", tag = "ApiTags::User")]
    async fn put_user(&self, user_id: Path<i64>, update: Json<UpdateUser>) -> UpdateUserResponse {
        // You can replace this logic with your own logic and Use database to store data.
        let mut users = self.users.lock().await;
        match users.get_mut(user_id.0 as usize) {
            Some(user) => {
                if let Some(name) = update.0.name {
                    user.name = name;
                }
                if let Some(password) = update.0.password {
                    user.password = password;
                }
                UpdateUserResponse::Ok
            }
            None => UpdateUserResponse::NotFound,
        }
    }
}
