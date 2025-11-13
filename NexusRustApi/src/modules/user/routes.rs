
// use serde::Serialize;
// modules/user/routes.rs
use actix_web::web;
// use actix_web::{HttpResponse, Responder};

// #[derive(Serialize)]
// pub struct SimpleRes {
//     pub code: i32,
//     pub msg: &'static str,
// }

// pub async fn register_users() -> impl Responder {
//     HttpResponse::Ok().json(SimpleRes {
//         code: 200,
//         msg: "注册成功",
//     })
// }
 // cfg.route("/register", web::post().to(register_users));


pub fn config(cfg: &mut web::ServiceConfig) {

  cfg.route("/register", web::post().to(crate::modules::user::handlers::register_users));

  cfg.route("/login", web::post().to(crate::modules::user::handlers::login_users));

  cfg.route("/getInfo", web::get().to(crate::modules::user::handlers::get_info));

  cfg.route("/system/users", web::get().to(crate::modules::user::handlers::get_all_users));
  cfg.route("/system/users", web::post().to(crate::modules::user::handlers::post_add_users));
  cfg.route("/system/users/{id}", web::get().to(crate::modules::user::handlers::get_user_detail));
  cfg.route("/system/users", web::put().to(crate::modules::user::handlers::put_update_users));
  cfg.route("/system/users/{id}", web::delete().to(crate::modules::user::handlers::delete_user));

  // 导出数据
  cfg.route("/system/users/export", web::post().to(crate::modules::user::handlers::export_users));

  // 导出模板
  cfg.route("/system/users/exporttemplate", web::post().to(crate::modules::user::handlers::export_users_template));

  // 导入数据
  cfg.route("/system/users/import", web::post().to(crate::modules::user::handlers::import_users));
}


// use actix_web::{web, HttpResponse, Result};
// use serde_json::json;
// use mysql::prelude::*;
// use serde::Serialize;

// #[derive(Serialize)]
// pub struct User {
//     id: u32,
//     name: String,
//     email: String,
// }
// pub async fn get_all_users() -> Result<HttpResponse> {
// 这里是你的用户获取逻辑
//     let users = vec![
//         json!({"id": 1, "name": "张三", "email": "zhangsan@example.com"}),
//         json!({"id": 2, "name": "李四", "email": "lisi@example.com"}),
//     ];  
//     Ok(HttpResponse::Ok().json(users))
// }
// // HTTP 处理函数
// pub async fn get_all_users() -> HttpResponse {
//     let mut conn = crate::database::create_pool();
    
//     let users: Vec<User> = match conn.query_map(
//         "SELECT id, name, email FROM sys_user",
//         |(id, name, email)| User { id, name, email }
//     ) {
//         Ok(users) => users,
//         Err(e) => {
//             eprintln!("Database query error: {}", e);
//             return HttpResponse::InternalServerError()
//                 .body("Failed to retrieve users");
//         }
//     };
    
//     HttpResponse::Ok().json(users)
// }
// // 配置用户相关的路由
// pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/users")
//             .route("", web::get().to(get_all_users))
//             // 可以继续添加其他用户相关路由
//             // .route("/{id}", web::get().to(get_user_by_id))
//             // .route("", web::post().to(create_user))
//             // .route("/{id}", web::put().to(update_user))
//             // .route("/{id}", web::delete().to(delete_user))
//     );
// }






