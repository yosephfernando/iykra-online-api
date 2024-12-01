mod about_modules;
mod corporate_modules;
mod config;

use actix_web::{App, HttpServer, web, HttpResponse};
use config::app_config::configure_app;
use config::db_conn::mongodb_con;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    /* Can add other db client connection. eg: postgres_client, mysql_client, etc */
    let mongodb_client = mongodb_con().await;
    HttpServer::new(move || {
        App::new()
            .app_data(mongodb_client.clone())
            .service(web::scope("/api").configure(configure_app))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}