pub mod dashboard;
pub mod search_or_register_thread;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::scope("/dashboard").configure(dashboard::init));
    cfg.service(search_or_register_thread::search_or_register_thread);
}
