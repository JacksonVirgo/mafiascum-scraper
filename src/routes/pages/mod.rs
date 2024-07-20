pub mod home;
pub mod scraper;
pub mod test;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(home::main);
    cfg.service(test::test);
    cfg.service(scraper::scraper);
}
