use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/getRouters", web::get().to(crate::modules::auth::handlers::get_routers));
}