use crate::{
  hash,
  types::{ApiKey, PartialUser, User, UserCredendials},
  Backend, Error, Result, Tokenizer,
};
use rocket::{
  response::status::Created,
  serde::json::{self, Json},
  State,
};
use rocket_okapi::openapi;

#[openapi]
#[post("/authenticate", data = "<credentials>")]
pub fn authenticate_user(
  credentials: std::result::Result<Json<UserCredendials>, json::Error<'_>>,
  tokenizer: &State<Tokenizer>,
  backend: &State<Backend>,
) -> Result<Json<ApiKey>> {
  let credentials = credentials?;

  backend
    .find_user(UserCredendials {
      password: hash(&credentials.password),
      ..credentials.into_inner()
    })
    .and_then(|user| {
      tokenizer.generate().map(|token| User {
        token: Some(token),
        ..user
      })
    })
    .and_then(|user| backend.update_user(user))
    .map(|user| {
      Json(ApiKey {
        token: user.token.unwrap(),
      })
    })
}

#[openapi]
#[post("/users", data = "<user>")]
pub fn add_user(
  user: std::result::Result<Json<User>, json::Error<'_>>,
  api_key: std::result::Result<ApiKey, Error>,
  backend: &State<Backend>,
) -> Result<Created<()>> {
  let user = user?;
  let _ = api_key?;

  let username = &user.username.clone();

  backend
    .add_user(User {
      password: hash(&user.password),
      ..user.into_inner()
    })
    .map(|_| Created::new(format!("/users/{}", username)))
}

#[openapi]
#[delete("/users/<username>")]
pub fn delete_user(
  username: String,
  api_key: std::result::Result<ApiKey, Error>,
  backend: &State<Backend>,
) -> Result<()> {
  let _ = api_key?;

  backend.delete_user(&username)
}

#[openapi]
#[get("/users")]
pub fn get_all_users(
  backend: &State<Backend>,
  api_key: std::result::Result<ApiKey, Error>,
) -> Result<Json<Vec<PartialUser>>> {
  let _ = api_key?;

  Ok(Json(
    backend
      .list_users()?
      .into_iter()
      .map(PartialUser::from)
      .collect(),
  ))
}
