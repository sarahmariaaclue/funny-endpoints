#[macro_use] extern crate rocket;

use std::env;

use anyhow::Result;

use rocket::State;
use rocket::http::Status;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub async fn find_by_id(id: i32, pool: &Pool<Postgres>) -> Result<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM my_table WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?;

        Ok(user)
    }
}

#[get("/<id>")]
async fn hello(pool: State<'_, Pool<Postgres>>, id: i32) -> Result<String, Status> {
    let user = User::find_by_id(id, &pool).await;

    match user {
        Ok(user) => Ok(format!("Hello {}!", &user.name)),
        _ => Err(Status::NotFound)
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    rocket::ignite()
        .mount("/", routes![hello])
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}