use anyhow::{Context, Result};
use simple_rocket_rs::{hash, types::User, Backend};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  #[structopt(subcommand)]
  command: Command,
  #[structopt(long, env)]
  pub database_url: String,
}

#[derive(StructOpt)]
enum Command {
  Create {
    #[structopt(short, long)]
    username: String,
    #[structopt(short, long)]
    email: String,
    #[structopt(short, long)]
    password: String,
    #[structopt(long)]
    is_admin: Option<bool>,
  },
}

fn main() -> Result<()> {
  let opt = Opt::from_args();
  let backend = Backend::new(&opt.database_url).with_context(|| "Cannot connect to database")?;

  match opt.command {
    Command::Create {
      username,
      email,
      password,
      is_admin,
    } => Ok(
      backend
        .add_user(User {
          username,
          email,
          password: hash(&password),
          // set user as admin by default
          is_admin: is_admin.unwrap_or(true),
          token: None,
        })
        .with_context(|| "Cannot create user")?,
    ),
  }
}
