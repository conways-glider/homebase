use actix_web::{middleware, App, HttpServer};

use homebase::app_config::config_app;

// use nested_routing::app_config::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let address = std::env::var("ADDRESS").unwrap_or("127.0.0.1".to_string());
    let port: u16  = std::env::var("PORT").unwrap_or("8080".to_string()).parse().expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .configure(config_app)
            .wrap(middleware::Logger::default())
    })
    .bind((address, port))?
    .run()
    .await
}
