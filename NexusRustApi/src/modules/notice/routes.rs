use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/system/notices", web::get().to(crate::modules::notice::handlers::get_list));
  cfg.route("/system/notices", web::post().to(crate::modules::notice::handlers::post_add));
  cfg.route("/system/notices/{id}", web::get().to(crate::modules::notice::handlers::get_detail));
  cfg.route("/system/notices", web::put().to(crate::modules::notice::handlers::put_update));
  cfg.route("/system/notices/{id}", web::delete().to(crate::modules::notice::handlers::del_delete));
}