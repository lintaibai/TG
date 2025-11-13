use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/jobs", web::get().to(crate::modules::job::handlers::get_list));
  cfg.route("/jobs", web::post().to(crate::modules::job::handlers::post_add));
  cfg.route("/jobs/{id}", web::get().to(crate::modules::job::handlers::get_detail));
  cfg.route("/jobs", web::put().to(crate::modules::job::handlers::put_update));
  cfg.route("/jobs/{id}", web::delete().to(crate::modules::job::handlers::del_delete));
}