use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};
use std::sync::{Arc, Mutex};

use rocksdb::{Options as RocksOptions, DB as RocksDB};

use leveldb::database::Database as LevelDB;
use leveldb::options::Options as LevelOptions;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::path::Path;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

pub async fn connect_to_pgsql() -> Result<PgPool, sqlx::Error> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            eprint!(
                "Coundn't load pgsql url going with default value error:{}",
                e
            );
            "postgres://admin:admin@localhost:5432/testdb".to_string()
        }
    };

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    Ok(pg_pool)
}

pub async fn connect_to_mongodb() -> Result<Database, mongodb::error::Error> {
    let uri = match std::env::var("MONGODB_URL") {
        Ok(url) => url,
        Err(e) => {
            eprint!(
                "Coundn't load mongodb url going with default value error:{}",
                e
            );
            "mongodb://admin:admin@localhost:27017".to_string()
        }
    };

    let mut client_options = ClientOptions::parse(&uri).await?;
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

pub async fn connect_to_surrealdb(
) -> Result<Surreal<surrealdb::engine::remote::ws::Client>, surrealdb::Error> {
    let surreal_url = match std::env::var("SURREALDB_URL") {
        Ok(url) => url,
        Err(e) => {
            eprint!(
                "Coundn't load surrealdb url going with default value error:{}",
                e
            );

            "localhost:8000".to_string()
        }
    };

    let db = Surreal::new::<Ws>(&surreal_url).await?;
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

pub fn connect_to_leveldb() -> Result<Arc<Mutex<LevelDB<i32>>>, Box<dyn std::error::Error>> {
    let mut opts = LevelOptions::new();
    opts.create_if_missing = true;

    let path = Path::new("./data/leveldb");
    std::fs::create_dir_all(path)?;

    let db = LevelDB::open(path, opts)?;
    println!("Successfully connected to LevelDB!");

    Ok(Arc::new(Mutex::new(db)))
}
