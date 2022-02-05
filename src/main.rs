use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use sqlx::SqlitePool;

#[get("/get_views")]
async fn get_views(pool: web::Data<SqlitePool>) -> impl Responder {
    let v: (i32,) = sqlx::query_as("SELECT views from views;")
        .fetch_one(pool.get_ref())
        .await
        .expect("");
    let v = v.0;
    HttpResponse::Ok().body(format!("hello world! num views: {}", v))
}
#[get("/")]
async fn root(pool: web::Data<SqlitePool>) -> impl Responder {
    sqlx::query("UPDATE views SET views = views +1;")
        .execute(pool.get_ref())
        .await
        .expect("failed to update view count");
    HttpResponse::Ok().body("<!DOCTYPE HTML>hello great content!")
}
#[actix_web::main]
async fn main() -> Result<()> {
    let conn = SqlitePool::connect("sqlite:test.db").await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(get_views)
            .service(root)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
