/*
 * Mosiac server source file (Database access)
 */
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use dotenvy::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

use std::net::SocketAddr;
/*use dotenvy::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use std::net::SocketAddr;*/

/*enum Command {
    Add { contents: String },
    Remove { id: u64 },
    Get { id: u64 },
}*/

/*#[derive(sqlx::FromRow)]
struct Post {
    id: u64,
    contents: Option<String>,
}*/

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();

    // setup connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .route(
            "/",
            get(using_connection_pool_extractor).post(using_connection_extractor),
        )
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    //let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    /*//let payload = Command::Add { contents: String::from("Test Post, Please Ignore") };
    let payload = Command::Remove { id: 4 };
    //let payload = Command::Get { id: 4 };
    match payload {
        Command::Add { contents } => {
            let post_id = add_post(&pool, contents).await?;
            println!("Added Post with id {}", post_id);
        }
        Command::Remove { id } => {
            if remove_post(&pool, id).await? {
                println!("Deleted Post with id {}", id);
            } else {
                println!("Could not Delete Post with id {}", id);
            }
        }
        Command::Get { id } => {
            let posts = get_posts(&pool, id).await?;
            if posts.len() > 0 {
                for post in posts {
                    println!("{}: {}", post.id, post.contents.unwrap());
                }
            } else {
                println!("No Posts Found!");
            }
        }
    }*/
    Ok(())
}

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
    State(pool): State<MySqlPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::MySql>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    MySqlPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = MySqlPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let mut conn = conn;
    sqlx::query_scalar("SELECT id, contents FROM post")
        .fetch_one(&mut conn)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/*async fn add_post(pool: &MySqlPool, contents: String) -> anyhow::Result<u64> {
    let post_id = sqlx::query!("INSERT INTO post ( contents ) VALUES ( ? )", contents)
        .execute(pool)
        .await?
        .last_insert_id();
    Ok(post_id)
}

async fn remove_post(pool: &MySqlPool, id: u64) -> anyhow::Result<bool> {
    let result = sqlx::query!("DELETE FROM post WHERE post.id = ?", id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(result > 0)
}

async fn get_posts(pool: &MySqlPool, id: u64) -> anyhow::Result<Vec<Post>> {
    let posts = sqlx::query_as!(Post, "SELECT id, contents FROM post WHERE post.id = ?", id)
        .fetch_all(pool)
        .await?;
    Ok(posts)
}*/
