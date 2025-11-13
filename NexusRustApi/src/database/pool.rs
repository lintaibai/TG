// 连接池实现
use mysql::*;
use dotenv::dotenv;
use std::env;

// 创建数据库连接
pub fn create_pool() -> mysql::Conn {
  dotenv().ok();
  
  let url = env::var("DATABASE_URL")
      .expect("DATABASE_URL environment variable not set");
  
  let opts = Opts::from_url(&url)
      .expect("Failed to parse database URL");
  
  Conn::new(opts)
      .expect("Failed to connect to MySQL database")
}