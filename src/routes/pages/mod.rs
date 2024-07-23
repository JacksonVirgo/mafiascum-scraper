pub mod dashboard;
pub mod home;
pub mod scraper;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(home::main);
    cfg.service(scraper::scraper);
    cfg.service(dashboard::dashboard);
}
