use actix_web::web;

use crate::handlers::test;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("")
                    .route(web::get().to(test::test)),
            )
    );
}
