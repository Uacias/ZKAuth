use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::args::Args;

pub async fn setup_surrealdb(args: Args) -> Surreal<Client> {
    let host = args.surrealdb_url.host_str().unwrap();
    let port = args.surrealdb_url.port().unwrap();
    let db_url = format!("{}:{}", host, port);
    let db = Surreal::new::<Ws>(db_url).await.unwrap();
    db.signin(Root {
        username: &args.surrealdb_username,
        password: &args.surrealdb_password,
    })
    .await
    .unwrap();

    db.use_ns(&args.surrealdb_namespace)
        .use_db(&args.surrealdb_database)
        .await
        .unwrap();

    db
}
