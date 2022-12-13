use dotenvy::dotenv;
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;
use std::env;
use yew::prelude::*;

#[derive(FromRow)]
struct Post {
    pub id: i32,
    pub contents: i32,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

//#[async_std::main]
async fn simulate_database_interaction() -> anyhow::Result<()> {
    let mut post_ids = vec![];
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    let test_additions: [&str; 2] = ["test1", "test2"];
    for test in test_additions.iter() {
        let post_result = add_post(&pool, test.to_string()).await?;
        post_ids.push(post_result);
    }
    let all_posts = get_posts(&pool).await?;
    for post in all_posts.iter() {
        println!("{}, {}", post.id, post.contents);
    }
    for id in post_ids.iter() {
        if remove_post(&pool, id.to_owned()).await? {
            println!("Post with id {} removed", id);
        } else {
            println!("Error removing post with id {}!", id);
        }
    }
    Ok(())
}

async fn add_post(pool: &MySqlPool, contents: String) -> anyhow::Result<u64> {
    // Insert the task, obtain the ID of this row
    let post_id = sqlx::query!("INSERT INTO posts (contents) VALUES (?)", contents)
        .execute(pool)
        .await?
        .last_insert_id();

    Ok(post_id)
}

async fn remove_post(pool: &MySqlPool, id: u64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!("DELETE FROM posts WHERE id = ?", id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected == 1)
}

async fn get_posts(pool: &MySqlPool) -> anyhow::Result<Vec<Post>> {
    let posts = sqlx::query_as!(
        Post,
        "SELECT * FROM (select (1) as id, (2) as contents) posts"
    )
    .fetch_all(pool)
    .await?;

    Ok(posts)
}

#[async_std::main]
async fn main() -> anyhow::Result<(), sqlx::Error> {
    dotenv().ok();
    simulate_database_interaction().await?;
    yew::Renderer::<App>::new().render();
}
