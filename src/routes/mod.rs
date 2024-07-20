use actix_web::web;
pub mod api;
pub mod not_found;
pub mod pages;
pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(pages::init);
    cfg.service(web::scope("/api").configure(api::init));
    cfg.default_service(web::route().to(not_found::not_found));
}
