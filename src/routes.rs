use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/roles").configure(crate::controllers::roles::config));
}
