use clap::Parser;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, env, default_value_t = Url::parse("http://0.0.0.0:8000").unwrap())]
    pub surrealdb_url: Url,

    #[arg(long, env, default_value_t = String::from("test"))]
    pub surrealdb_database: String,

    #[arg(long, env, default_value_t = String::from("test"))]
    pub surrealdb_namespace: String,

    #[arg(long, env, default_value_t = String::from("root"))]
    pub surrealdb_password: String,

    #[arg(long, env, default_value_t = String::from("root"))]
    pub surrealdb_username: String,
}
