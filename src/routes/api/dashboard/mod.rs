pub mod player_data;
pub mod setup_data;
pub mod vote_data;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(player_data::player_data);
    cfg.service(vote_data::vote_data);
    cfg.service(setup_data::setup_data);
}