pub mod scrape_activity_page;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(scrape_activity_page::scrape_activity_page);
}
