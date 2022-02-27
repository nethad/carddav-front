use actix_web::{
    middleware::{self, Logger},
    App, HttpServer,
};
use carddav_front::{routes::routing_configuration, setup_app_data};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .wrap(Logger::default())
            .configure(routing_configuration)
            .data(setup_app_data())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
