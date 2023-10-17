mod routes;
mod middleware;

use actix_web::{App,HttpServer};

use routes::route::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}