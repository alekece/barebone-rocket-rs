#[cfg(all(feature = "postgres", feature = "sqlite"))]
compile_error!("features `crate/postgres` and `crate/sqlite` are mutually exclusive");

use crate::{
  types::{User, UserCredendials},
  Error, Result,
};
use diesel::{r2d2::ConnectionManager, ExpressionMethods, QueryDsl, RunQueryDsl};
use r2d2::{Pool, PooledConnection};

#[cfg(feature = "postgres")]
type Connection = diesel::PgConnection;

#[cfg(feature = "sqlite")]
type Connection = diesel::SqliteConnection;

pub struct Backend {
  connection_pool: Pool<ConnectionManager<Connection>>,
}

impl Backend {
  pub fn new(database_url: &str) -> Result<Self> {
    let manager = ConnectionManager::<Connection>::new(database_url);

    Ok(Self {
      connection_pool: Pool::builder()
        .min_idle(Some(1))
        .max_size(10)
        .build(manager)?,
    })
  }

  pub fn find_user(&self, credentials: UserCredendials) -> Result<User> {
    use crate::schema::users::dsl::{self, users};

    let conn = self.get_connection()?;

    users
      .find(credentials.username)
      .filter(dsl::password.eq(credentials.password))
      .first(&conn)
      .map_err(|e| match e {
        diesel::result::Error::NotFound => Error::NotFound,
        _ => e.into(),
      })
  }

  pub fn find_user_by_token(&self, token: &str) -> Result<User> {
    use crate::schema::users::dsl::{self, users};

    let conn = self.get_connection()?;

    users
      .filter(dsl::token.eq(token))
      .first(&conn)
      .map_err(|e| match e {
        diesel::result::Error::NotFound => Error::NotFound,
        _ => e.into(),
      })
  }

  pub fn add_user(&self, new_user: User) -> Result<()> {
    use crate::schema::users::dsl::users;

    let conn = self.get_connection()?;

    Ok(
      diesel::insert_into(users)
        .values(new_user)
        .execute(&conn)
        .map(|_| ())?,
    )
  }

  pub fn update_user(&self, user: User) -> Result<User> {
    let conn = self.get_connection()?;

    match diesel::update(&user).set(&user).execute(&conn)? {
      0 => Err(Error::NotFound),
      1 => Ok(user),
      i => Err(Error::InvalidResult(format!(
        "Updated {} rows in users table instead of exactly 1",
        i,
      ))),
    }
  }

  pub fn delete_user(&self, username: &str) -> Result<()> {
    use crate::schema::users::dsl::users;

    let conn = self.get_connection()?;

    match diesel::delete(users.find(username)).execute(&conn)? {
      0 => Err(Error::NotFound),
      1 => Ok(()),
      i => Err(Error::InvalidResult(format!(
        "Deleted {} rows in users table instead of exactly 1",
        i,
      ))),
    }
  }

  pub fn list_users(&self) -> Result<Vec<User>> {
    use crate::schema::users::dsl::users;

    let conn = self.get_connection()?;

    Ok(users.load(&conn)?)
  }

  fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<Connection>>> {
    Ok(self.connection_pool.get()?)
  }
}
