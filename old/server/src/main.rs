/*
 * Mosiac server source file (Database access)
 */
use dotenvy::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, FromRow};
use uuid::Uuid;

#[derive(Clone, FromRow, Debug)]
struct User {
    uuid: Uuid,
    username: String,
}

async fn ensure_database(db_connection_str: &str){
    if !Sqlite::database_exists(db_connection_str).await.unwrap_or(false){
        match Sqlite::create_database(db_connection_str).await{
            Ok(_) => println!("Success creating database {db_connection_str}"),
            Err(error) => panic!("error: {error}"),
        }
    }
    else{
        println!("Database {db_connection_str} already exists!");
    }
}

async fn add_dummy_user(pool: &SqlitePool) -> Result<(), sqlx::Error>{
    let result = sqlx::query!(
        r#"
INSERT INTO users (uuid, username)
VALUES (?)
        "#,
        Uuid::new_v4(),
        "ClarkMyWords"
        )
        .execute(pool)
        .await?;
    Ok(result)
}

async fn populate_user_table(pool: &SqlitePool) -> Result<(), sqlx::Error>{
    let result = sqlx::query!(
        r#"
CREATE TABLE IF NOT EXISTS users
(uuid UUID PRIMARY KEY NOT NULL, username VARCHAR(250) NOT NULL);
        "#).
        execute(pool).
        await?;
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    dotenv().ok();
    let db_connection_str = std::env::var("DATABASE_URL").unwrap();

    ensure_database(&db_connection_str).await;

    // setup connection pool
    let pool = SqlitePool::connect(&db_connection_str).await.expect("Could not connect to sqlite db");

    let result = populate_user_table(&pool)
        .await
        .expect("Failed to populate the user table");

    let result = add_dummy_user(&pool)
        .await
        .expect("Failed to add dummy user to the user table");

    Ok(())
}
