use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {

  // 字典类型
  cfg.route("/system/dictType", web::get().to(crate::modules::dict::handlers::get_list));
  cfg.route("/system/dictType", web::post().to(crate::modules::dict::handlers::post_add));
  cfg.route("/system/dictType", web::put().to(crate::modules::dict::handlers::put_update));
  cfg.route("/system/dictType/optionselect", web::get().to(crate::modules::dict::handlers::get_all_dict_type));
  cfg.route("/system/dictType/{id}", web::get().to(crate::modules::dict::handlers::get_detail));
  cfg.route("/system/dictType/{id}", web::delete().to(crate::modules::dict::handlers::del_delete));

  // 字典数据
  cfg.route("/system/dictData", web::get().to(crate::modules::dict::handlers::get_list_data));
  cfg.route("/system/dictData", web::post().to(crate::modules::dict::handlers::post_add_data));
  cfg.route("/system/dictData", web::put().to(crate::modules::dict::handlers::put_update_data));
  cfg.route("/system/dictData/optionselect", web::get().to(crate::modules::dict::handlers::get_all_dict_data));
  cfg.route("/system/dictData/{id}", web::get().to(crate::modules::dict::handlers::get_detail_data));
  cfg.route("/system/dictData/{id}", web::delete().to(crate::modules::dict::handlers::del_delete_data));
}