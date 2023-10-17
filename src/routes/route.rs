use actix_web::{web,HttpResponse};
use actix_web_lab::middleware::from_fn;
use crate::middleware::middle::middle;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .wrap(from_fn(middle))
            .route(web::get().to(|| async { HttpResponse::Ok().body("success") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}