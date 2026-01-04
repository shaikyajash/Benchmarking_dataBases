use mongodb::{
    Client, Database,
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};
use std::sync::{Arc, Mutex};

use rocksdb::{DB as RocksDB, Options as RocksOptions};

use rusty_leveldb::{DB as LevelDB, Options as LevelOptions};
use sqlx::{PgPool, postgres::PgPoolOptions};
use surrealdb::{Surreal, engine::remote::ws::Ws, opt::auth::Root};

pub async fn connect_to_pgsql() -> Result<PgPool, sqlx::Error> {
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://admin:admin@localhost:5432/testdb")
        .await?;

    Ok(pg_pool)
}

pub async fn connect_to_mongodb() -> Result<Database, mongodb::error::Error> {
    let uri = "mongodb://admin:admin@localhost:27017";
    let mut client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;

    // Send a ping to confirm a successful connection
    client
        .database("testdb")
        .run_command(doc! { "ping": 1 })
        .await?;

    println!("Pinged your deployment. You successfully connected to MongoDB!");

    //Return the database for use
    Ok(client.database("testdb"))
}

pub async fn connect_to_surrealdb()
-> Result<Surreal<surrealdb::engine::remote::ws::Client>, surrealdb::Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("testns").use_db("testdb").await?;
    println!("Successfully connected to SurrealDB!");
    Ok(db)
}

pub fn connect_to_rocksdb() -> Result<RocksDB, rocksdb::Error> {
    let mut opts = RocksOptions::default();

    opts.create_if_missing(true);

    let db = RocksDB::open(&opts, "./data/rocksdb")?;
    println!("Successfully connected to RocksDB!");

    Ok(db)
}

pub fn connect_to_leveldb() -> Result<Arc<Mutex<LevelDB>>, rusty_leveldb::Status> {
    let mut opts = LevelOptions::default();
    opts.create_if_missing = true;

    let db = LevelDB::open("./data/leveldb", opts)?;
    println!("Successfully connected to LevelDB!");
    Ok(Arc::new(Mutex::new(db)))
}
