use actix_web::{web, App, HttpServer, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::Serialize;
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Debug)]
struct User {
    id: i32,
    name: String,
}

async fn get_users() -> impl Responder {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match Pool::new(database_url.as_str()) {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Error creating pool: {:?}", err);
            return web::Json(vec![]);
        }
    };

    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error getting connection: {:?}", err);
            return web::Json(vec![]);
        }
    };

    let users: Vec<User> = match conn.query_map(
        "SELECT id, name FROM users",
        |(id, name): (i32, String)| User { id, name },
    ) {
        Ok(users) => users,
        Err(err) => {
            eprintln!("Error querying users: {:?}", err);
            return web::Json(vec![]);
        }
    };

    web::Json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}