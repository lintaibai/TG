
use actix_cors::Cors;
use actix_web::{App, HttpServer, Responder,HttpResponse,web,http::header};
use dotenv::dotenv;
use sqlx::MySqlPool;
use std::env;

// use std::net::SocketAddr; // 服务器地址

use log::{info}; //debug, warn, error
use env_logger::Builder; //初始化日志

// 申明模块
pub mod common;
pub mod modules;
pub mod database;

// 文件服务
use actix_files as fs;
use std::fs::create_dir_all; // 创建目录


async fn welcome() -> impl Responder {
    // 获取环境变量
    let base_url = env::var("BASE_URL").expect("BASE_URL environment variable not set");

    let base_url_prefix = env::var("BASE_URL_PREFIX").expect("BASE_URL_PREFIX environment variable not set");
    
    // 拼接完整的URL
    let full_url = format!("{}{}", base_url, base_url_prefix);
  
    //欢迎信息
    let welcome_msg = format!("欢迎使用NexusRust-API，后台接口服务地址 {}", full_url);
    
    log::info!("这是一条中文信息");

    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/plain; charset=utf-8"))
        .body(welcome_msg)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // 一定要在读取环境变量之前调用

    // 从环境变量中获取配置
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8888".to_string());

    // 获取环境变量
    let base_url = env::var("BASE_URL").expect("BASE_URL environment variable not set");

    let base_url_prefix = env::var("BASE_URL_PREFIX").expect("BASE_URL_PREFIX environment variable not set");
    
    // 拼接完整的URL
    let full_url = format!("{}{}", base_url, base_url_prefix);
  
    // 创建服务器地址
    let addr= format!("{}:{}", host, port);

    //欢迎信息
    let welcome_msg = format!("==NexusRust-API==后台接口地址 {}", full_url);

    // 显式设置日志级别和输出格式
    Builder::new()
        .parse_filters("info") // 设置日志级别为 info
        .init(); // 初始化日志记录器

    info!("日志系统已初始化！！！");

     // println!("服务器地址: {}", addr);
    info!("服务器地址: {}", addr);

    // 欢迎信息
    info!("{}", welcome_msg);

    // 确保上传目录存在
    let upload_dirs = [
        "./uploads",
        "./uploads/images",
        "./uploads/documents",
        "./uploads/videos",
        "./uploads/others"
    ];
    for dir in &upload_dirs {
        if let Err(e) = create_dir_all(dir) {
            eprintln!("创建目录 {} 失败: {}", dir, e);
            // 生产环境中可能需要更严格的处理
        }
    }
    info!("图片服务器已准备！");
    
    // 1. 初始化数据库连接池
    let database_url = env::var("DATABASE_URL").unwrap(); 
    
    // 创建 MySQL 异步连接池
    let pool = MySqlPool::connect(&database_url).await.unwrap();
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header(); // 允许所有来源
        App::new()
            // 添加 CORS 中间件
            .wrap(cors)
            // 2. 注入数据库连接池
            .app_data(web::Data::new(pool.clone()))
            // 3. 注册模块路由加前缀
            .service(
                fs::Files::new("/uploads/images", "./uploads/images")
                    .prefer_utf8(true)
                    .show_files_listing() // 开发环境使用，生产环境应移除
            )
            // 可以添加其他静态文件目录
            .service(
                fs::Files::new("/uploads/documents", "./uploads/documents")
            )
            .service(
                web::scope(&base_url_prefix) // 使用环境变量中的前缀
                    .route("", web::get().to(welcome))  // 添加空路径路由
                    .route("/", web::get().to(welcome))  // 添加根路径路由
                    .configure(modules::user::routes::config)  // 用户模块
                    .configure(modules::upload::routes::config) // 上传模块
                    .configure(modules::role::routes::config) // 角色模块
                    .configure(modules::dict::routes::config) // 字典模块
                    .configure(modules::menu::routes::config) // 菜单模块
                    .configure(modules::auth::routes::config) // 权限模块
                    .configure(modules::dept::routes::config) // 部门模块
                    .configure(modules::notice::routes::config) // 通知模块
                    .configure(modules::job::routes::config) //职位模块
            )
            // 3. 注册路由
            .route("/", web::get().to(welcome))
            .route("", web::get().to(welcome))
    })
    .bind(addr)?
    .run()
    .await
}
