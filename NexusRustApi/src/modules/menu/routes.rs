use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/system/menus", web::get().to(crate::modules::menu::handlers::get_list));
  cfg.route("/system/menus", web::post().to(crate::modules::menu::handlers::post_add));
  cfg.route("/system/menus/{id}", web::get().to(crate::modules::menu::handlers::get_detail));
  cfg.route("/system/menus", web::put().to(crate::modules::menu::handlers::put_update));
  cfg.route("/system/menus/{id}", web::delete().to(crate::modules::menu::handlers::del_delete));
}