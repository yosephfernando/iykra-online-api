use actix_web::{web, HttpResponse};
use crate::about_modules::controllers::about;
use crate::corporate_modules::controllers::{training, trustedby};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/about")
            .route(web::get().to(about::index))
            .route(web::put().to(about::update))
            .route(web::post().to(about::create))
            .route(web::delete().to(about::delete))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );

    cfg.service(
        web::resource("/corporate/training")
            .route(web::get().to(training::index))
            .route(web::put().to(training::update))
            .route(web::post().to(training::create))
            .route(web::delete().to(training::delete))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );

    cfg.service(
        web::resource("/corporate/trusted-by")
            .route(web::get().to(trustedby::index))
            .route(web::put().to(trustedby::update))
            .route(web::post().to(trustedby::create))
            .route(web::delete().to(trustedby::delete))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}