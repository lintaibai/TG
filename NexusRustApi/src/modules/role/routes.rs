use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/system/roles", web::get().to(crate::modules::role::handlers::get_list));
  cfg.route("/system/roles", web::post().to(crate::modules::role::handlers::post_add));
  cfg.route("/system/roles/{id}", web::get().to(crate::modules::role::handlers::get_detail));
  cfg.route("/system/roles", web::put().to(crate::modules::role::handlers::put_update));
  cfg.route("/system/roles/{id}", web::delete().to(crate::modules::role::handlers::del_delete));
}