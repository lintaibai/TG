use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {

  // 文章类型
  cfg.route("/system/articles", web::get().to(crate::modules::dict::handlers::get_list));
  cfg.route("/system/articles", web::post().to(crate::modules::dict::handlers::post_add));
  cfg.route("/system/articles", web::put().to(crate::modules::dict::handlers::put_update));
  cfg.route("/system/articles/optionselect", web::get().to(crate::modules::dict::handlers::get_all_dict_type));
  cfg.route("/system/articles/{id}", web::get().to(crate::modules::dict::handlers::get_detail));
  cfg.route("/system/articles/{id}", web::delete().to(crate::modules::dict::handlers::del_delete));
}