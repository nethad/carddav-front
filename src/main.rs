use actix_web::{App, HttpServer};
use carddav_front::routing_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routing_configuration))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
