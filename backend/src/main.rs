pub mod routes;
pub mod schema;
pub mod models;

use routes::create_route::create_short_url;
use actix_web::{App, HttpServer};

use crate::routes::get_route::get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_short_url)
            .service(get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}