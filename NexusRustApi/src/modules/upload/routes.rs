use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/upload/image", web::post().to(crate::modules::upload::handlers::upload_img));
}
