use actix_web::{App, HttpServer};
use carddav_front::{routes::routing_configuration, setup_app_data};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routing_configuration)
            .data(setup_app_data())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
