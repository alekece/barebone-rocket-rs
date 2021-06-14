use rocket::routes;
use simple_rocket_rs::{routes, Backend, Result, Tokenizer};
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  #[structopt(long, env)]
  pub database_url: String,
  #[structopt(long, env = "JWT_SECRET", hide_env_values = true)]
  pub secret_key: Option<String>,
  #[structopt(long, default_value = "1 day", env = "JWT_EXPIRES_IN", parse(try_from_str = parse_duration::parse))]
  pub token_expiration: Duration,
}

#[rocket::main]
async fn main() -> Result<()> {
  let opt = Opt::from_args();

  rocket::build()
    .manage(Tokenizer::new(
      opt.token_expiration,
      opt.secret_key.as_deref(),
    ))
    .manage(Backend::new(&opt.database_url)?)
    .mount(
      "/",
      routes![
        routes::authenticate_user,
        routes::add_user,
        routes::delete_user,
        routes::get_all_users
      ],
    )
    .launch()
    .await
    .map_err(Box::from)?;

  Ok(())
}
