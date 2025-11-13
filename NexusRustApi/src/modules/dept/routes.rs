use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/system/depts", web::get().to(crate::modules::dept::handlers::get_list));
  cfg.route("/system/depts", web::post().to(crate::modules::dept::handlers::post_add));
  cfg.route("/system/depts/{id}", web::get().to(crate::modules::dept::handlers::get_detail));
  cfg.route("/system/depts", web::put().to(crate::modules::dept::handlers::put_update));
  cfg.route("/system/depts/{id}", web::delete().to(crate::modules::dept::handlers::del_delete));

  // 导出模板
  // cfg.route("/system/depts/exporttemplate", web::post().to(crate::modules::dept::handlers::post_export_template));

  // 导出数据
  cfg.route("/system/depts/export", web::post().to(crate::modules::dept::handlers::post_export));

  // 导入数据
  // cfg.route("/system/depts/import", web::post().to(crate::modules::user::handlers::post_import));
}