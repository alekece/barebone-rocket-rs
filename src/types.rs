use rocket::{
  http::Status,
  outcome::{try_outcome, IntoOutcome},
  request::{FromRequest, Outcome, Request},
  State,
};

use crate::{schema::*, Backend, Error, Tokenizer};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
  Debug, Clone, Queryable, Identifiable, Insertable, Serialize, Deserialize, AsChangeset, JsonSchema,
)]
#[table_name = "users"]
#[primary_key(username)]
pub struct User {
  pub username: String,
  pub email: String,
  pub password: String,
  pub is_admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PartialUser {
  pub username: String,
  pub email: String,
  pub is_admin: bool,
}

impl From<User> for PartialUser {
  fn from(user: User) -> Self {
    Self {
      username: user.username,
      email: user.email,
      is_admin: user.is_admin,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserCredendials {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewPassword {
  pub current: String,
  pub new: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ApiKey {
  pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
  type Error = Error;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let tokenizer = try_outcome!(request
      .guard::<&State<Tokenizer>>()
      .await
      .map_failure(|_| (Status::InternalServerError, Error::InternalError)));

    let backend = try_outcome!(request
      .guard::<&State<Backend>>()
      .await
      .map_failure(|_| (Status::InternalServerError, Error::InternalError)));

    request
      .headers()
      .get_one("Authorization")
      .map(|header| header.split("Bearer").collect::<Vec<_>>())
      .ok_or(Error::UnauthenticatedUser)
      .and_then(|bearer| {
        let token = bearer
          .as_slice()
          .get(1)
          .map(|token| token.trim())
          .unwrap_or_default();

        tokenizer.verify(token).map(|_| token.to_string())
      })
      .and_then(|token| match backend.find_user_by_token(&token) {
        Ok(user) if user.is_admin => Ok(ApiKey { token }),
        Ok(_) => Err(Error::ForbiddenAccess),
        Err(_) => Err(Error::UnauthenticatedUser),
      })
      .into_outcome(Status::BadRequest)
  }
}
