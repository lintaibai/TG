

// 测试接口
// pub async fn post_add_users() -> HttpResponse {
//     HttpResponse::Ok().json(ApiResponse {
//         code: 200,
//         msg: "接口信息",
//         data:  None::<()>,
//     })
// }

// 不分离版本


   // let exactquery: HashMap<String, String> = [
    //     //  ("age".to_string(), 6.to_string())    // 精确查询
    // ]
    // .iter()
    // .cloned()
    // .collect();

    // let likequery: HashMap<String, String> = [
    //     ("age".to_string(), "6".to_string()) // 模糊查询
    // ]
    // .iter()
    // .cloned()
    // .collect();


// filter: Option<web::Query<QueryParams>>, // 接受动态查询条件
// 精确查询
// let exactquery: HashMap<String, String> = [
//     ("age".to_string(), query.age.unwrap_or(0).to_string()),  // 默认 age 为 0
//     ("sex".to_string(), query.sex.clone().unwrap_or("Unknown".to_string()))  // 默认 sex 为 "Unknown"
// ]
// .iter()
// .cloned()
// .collect();

// // 模糊查询
// let likequery: HashMap<String, String> = [
//     ("name".to_string(), query.name.clone().unwrap_or("Unknown".to_string())) // 默认 name 为 "Unknown"
// ]
// .iter()
// .cloned()
// .collect();





//公共查询方法--版本1
// #[allow(dead_code)]
// pub async fn list_api_page(
//     pool: web::Data<MySqlPool>,
//     query: web::Query<Pagination>,
//     table_name: &str, // 表名
// ) -> impl Responder {

//     let page = query.page_num.unwrap_or(1);
//     let page_size = query.page_size.unwrap_or(10);
//     let offset = (page - 1) * page_size;

//     // info!("收到查询请求{}",query.page_num.unwrap_or(1));

//     // 查询总数
//     // 动态构建查询语句
//     let querytotal = format!("SELECT COUNT(*) FROM {}", table_name);

//     let total: (i64,) = match sqlx::query_as(&querytotal)
//         .fetch_one(pool.get_ref())
//         .await
//     {
//         Ok(count) => count,
//         Err(e) => {
//             eprintln!("查询总数失败: {:?}", e);
//             return HttpResponse::InternalServerError().body("查询总数失败");
//         }
//     };

//     // 分页查询
//     let query = format!("SELECT * FROM {} LIMIT ? OFFSET ?", table_name);
//     let users = sqlx::query_as::<_, User>(&query)
//         .bind(page_size as i64)
//         .bind(offset as i64)
//         .fetch_all(pool.get_ref())
//         .await;

//     // let users = sqlx::query_as::<_, User>("SELECT * FROM sys_user")
//     //     .fetch_all(pool.get_ref())
//     //     .await;
//     match users {
//         Ok(list) => HttpResponse::Ok().json(
//             ListResponse {
//                 code: 200,
//                 msg: "查询成功",
//                 data: Some(list),
//                 total: total.0,
//             }
//         ),
//         Err(e) => {
//             eprintln!("数据库查询失败: {:?}", e);
//             HttpResponse::InternalServerError().body("数据库查询失败")
//         }
//     }
// }

//公共查询方法--版本2
// #[allow(dead_code)]
// pub async fn list_api_page(
//     pool: web::Data<MySqlPool>,
//     query: web::Query<QueryParams>,
//     _filter: Option<web::Query<ListQuery>>, // 动态查询条件
//     table_name: &str,                      // 表名
//     exactquery: HashMap<String, String>,   // 精确查询条件
//     likequery: HashMap<String, String>,    // 模糊查询条件
// ) -> impl Responder {
//     let page = query.page_num.unwrap_or(1);
//     let page_size = query.page_size.unwrap_or(10);
//     let offset = (page - 1) * page_size;

//     // 查询总数的 SQL
//     let mut query_total = format!("SELECT COUNT(*) FROM {}", table_name);

//     // 查询条件
//     let mut query_params: Vec<String> = Vec::new();  // 存储具体类型
//     let mut where_clauses = Vec::new();

//     // 处理精确查询（exactquery）
//     for (field, value) in exactquery {
//         where_clauses.push(format!("{} = ?", field)); // 精确查询
//         query_params.push(value); // 使用具体类型
//     }

//     // 处理模糊查询（likequery）
//     for (field, value) in likequery {
//         where_clauses.push(format!("{} LIKE ?", field)); // 模糊查询
//         query_params.push(format!("%{}%", value)); // 模糊查询时需要加上 %
//     }

//     // 如果有查询条件，添加 WHERE 子句
//     if !where_clauses.is_empty() {
//         query_total.push_str(" WHERE ");
//         query_total.push_str(&where_clauses.join(" AND "));
//     }

//     // 创建查询总数
//     let mut query = sqlx::query_as::<_, (i64,)>(&query_total);

//     // 遍历并逐个绑定参数
//     for value in query_params.iter() {
//         query = query.bind(value);  // 直接绑定具体的值
//     }

//     let total: (i64,) = match query.fetch_one(pool.get_ref()).await {
//         Ok(count) => count,
//         Err(e) => {
//             eprintln!("查询总数失败: {:?}", e);
//             return HttpResponse::InternalServerError().body("查询总数失败");
//         }
//     };

//     // 分页查询 SQL
//     let mut query = format!("SELECT * FROM {}", table_name);

//     // 如果有查询条件，拼接 WHERE 子句
//     if !where_clauses.is_empty() {
//         query.push_str(" WHERE ");
//         query.push_str(&where_clauses.join(" AND "));
//     }

//     // 添加分页条件
//     query.push_str(" LIMIT ? OFFSET ?");

//     // 合并查询参数
//     query_params.push(page_size.to_string());  // 添加分页参数
//     query_params.push(offset.to_string());    // 添加偏移量
//     println!("mysql查询条件{}",query);
    
//     // 创建查询
//     let mut query = sqlx::query_as::<_, User>(&query);
//     for value in query_params.iter() {
//         query = query.bind(value);  // 绑定分页参数
//     }

//     // 执行查询
//     let users = match query.fetch_all(pool.get_ref()).await {
//         Ok(list) => list,
//         Err(e) => {
//             eprintln!("数据库查询失败: {:?}", e);
//             return HttpResponse::InternalServerError().body("数据库查询失败");
//         }
//     };

//     // 返回结果
//     HttpResponse::Ok().json(ListResponse {
//         code: 200,
//         msg: "查询成功",
//         data: Some(users),
//         total: total.0,
//     })
// }


// 添加一个字段映射函数
// fn map_field_name(column_name: &str) -> String {
//     match column_name {
//         "user_id" => "userId".to_string(),
//         "user_height" => "userHeight".to_string(),
//         "user_weight" => "userWeight".to_string(),
//         "created_at" => "createdAt".to_string(),
//         "updated_at" => "updatedAt".to_string(),
//         _ => column_name.to_string(),
//     }
// }


// 查询用户详情
// pub async fn get_user_detail(
//     pool: web::Data<MySqlPool>,
//     path: web::Path<i32>, // user_id
// ) -> HttpResponse {
//     let user_id = path.into_inner();

//     let result = sqlx::query_as::<_, User>("SELECT * FROM sys_user WHERE user_id = ?")
//         .bind(user_id)
//         .fetch_one(pool.get_ref())
//         .await;

//     match result {
//         Ok(user) => {
//             let response = ApiDetailResponse {
//                 code: 200,
//                 msg: "查询成功",
//                 data: user, // 返回数据格式可根据实际需求进行修改
//             };
//             HttpResponse::Ok().json(response)
//         }
//         Err(sqlx::Error::RowNotFound) => {
//             let response = ApiDetailResponse {
//                 code: 404,
//                 msg: "用户不存在",
//                 data: (),
//             };
//             HttpResponse::NotFound().json(response)
//         }
//         Err(e) => {
//             eprintln!("查询用户详情失败: {:?}", e);
//             let response = ApiDetailResponse {
//                 code: 500,
//                 msg: "查询失败",
//                 data: (),
//             };
//             HttpResponse::InternalServerError().json(response)
//         }
//     }
// }




// 查询用户详情
// pub async fn get_user_detail(
//     pool: web::Data<MySqlPool>,
//     path: web::Path<i32>, // user_id
// ) -> HttpResponse {
//     let user_id = path.into_inner();

//     let result = sqlx::query_as::<_, User>("SELECT * FROM sys_user WHERE user_id = ?")
//         .bind(user_id)
//         .fetch_one(pool.get_ref())
//         .await;

//     match result {
//         Ok(user) => {
//             let response = ApiDetailResponse {
//                 code: 200,
//                 msg: "查询成功",
//                 data: user, // 返回数据格式可根据实际需求进行修改
//             };
//             HttpResponse::Ok().json(response)
//         }
//         Err(sqlx::Error::RowNotFound) => {
//             let response = ApiDetailResponse {
//                 code: 404,
//                 msg: "用户不存在",
//                 data: (),
//             };
//             HttpResponse::NotFound().json(response)
//         }
//         Err(e) => {
//             eprintln!("查询用户详情失败: {:?}", e);
//             let response = ApiDetailResponse {
//                 code: 500,
//                 msg: "查询失败",
//                 data: (),
//             };
//             HttpResponse::InternalServerError().json(response)
//         }
//     }
// }





//更新
// #[derive(Serialize)]
// pub struct Response {
//     code: i32,
//     msg: String,
// }
// pub async fn put_update_users(
//     pool: web::Data<Pool<MySql>>, 
//     item: web::Json<UpdateUserRequest>
// ) -> impl Responder {
//     let result = sqlx::query!(
//         "UPDATE sys_user SET age = ? WHERE user_id = ?",
//         item.age,
//         item.user_id
//     )
//     .execute(pool.get_ref())
//     .await;

//     match result {
//         Ok(_) => HttpResponse::Ok().json(Response {
//             code: 200,
//             msg: "更新成功!".to_string(),
//         }),
//         Err(e) => {
//             eprintln!("数据库更新失败: {:?}", e);
//             HttpResponse::InternalServerError().json(Response {
//                 code: 500,
//                 msg: "更新失败!".to_string(),
//             })
//         }
//     }
// }

// 删除用户
// pub async fn delete_user(
//     pool: web::Data<Pool<MySql>>, 
//     id: web::Path<i32>  // Path 中包含了用户的 ID
// ) -> impl Responder {
//     // 执行删除操作
//     let result = sqlx::query!(
//         "DELETE FROM sys_user WHERE user_id = ?",
//         *id  // 解引用 Path 获取具体的值
//     )
//     .execute(pool.get_ref())
//     .await;
//     match result {
//         Ok(_) => HttpResponse::Ok().json(BasicResponse {
//             code: 200,
//             msg: "删除成功!",
//         }),
//         Err(e) => {
//             eprintln!("数据库删除失败: {:?}", e);
//             HttpResponse::InternalServerError().json(BasicResponse {
//                 code: 500,
//                 msg: "删除失败!",
//             })
//         }
//     }
// }































// 查找用户分页
// pub async fn (pool: &PgPool) {
    // pub async fn get_all_users(pool: web::Data<Pool<MySql>>) -> impl Responder {
    //     // 定义 SQL 查询
    //     let query = Some("SELECT * FROM sys_user");  // 查询用户表
    
    //     // 调用 list_api 函数获取查询结果
    //     match list_api(pool.get_ref(), query).await {
    //         Ok(response) => {
    //             print!("response: {:?}", response);
    //             // 如果查询成功，返回结果
    //             HttpResponse::Ok().json(response)
    //         }
    //         Err(e) => {
    //             // 如果查询失败，返回错误信息
    //             HttpResponse::InternalServerError().json(BasicResponse  {
    //                 code: 200,
    //                 msg: "数据库查询失败",
    //             })
    //         }
    //     }
    // }
 //   pub async fn get_all_users(pool: web::Data<Pool<MySql>>) -> impl Responder {
    //     // 定义 SQL 查询
    //     // let query = Some("SELECT * FROM sys_user");  // 查询用户表
    //     // list_api_page(pool, query).await;
    //     // 调用 list_api 函数获取查询结果
    //     // match list_api_page(pool.get_ref(), query).await {
    //     //     Ok(response) => {
    //     //         print!("response: {:?}", response);
    //     //         // 如果查询成功，返回结果
    //     //         HttpResponse::Ok().json(response)
    //     //     }
    //     //     Err(e) => {
    //     //         // 如果查询失败，返回错误信息
    //     //         HttpResponse::InternalServerError().json(BasicResponse  {
    //     //             code: 200,
    //     //             msg: "数据库查询失败",
    //     //         })
    //     //     }
    //     // }
    // }

         
        // let page = query.page_num.unwrap_or(1);
        // let page_size = query.page_size.unwrap_or(10);
        // let offset = (page - 1) * page_size;
    
        // // info!("收到查询请求{}",query.page_num.unwrap_or(1));
    
        // // 查询总数
        // let total: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM sys_user")
        //     .fetch_one(pool.get_ref())
        //     .await
        // {
        //     Ok(count) => count,
        //     Err(e) => {
        //         eprintln!("查询总数失败: {:?}", e);
        //         return HttpResponse::InternalServerError().body("查询总数失败");
        //     }
        // };
    
    
        // // 分页查询
        // let users = sqlx::query_as::<_, User>("SELECT * FROM sys_user LIMIT ? OFFSET ?")
        //     .bind(page_size as i64)
        //     .bind(offset as i64)
        //     .fetch_all(pool.get_ref())
        //     .await;
    
        // // let users = sqlx::query_as::<_, User>("SELECT * FROM sys_user")
        // //     .fetch_all(pool.get_ref())
        // //     .await;
    
        // match users {
        //     Ok(list) => HttpResponse::Ok().json(
        //      ListResponse {
        //         code: 200,
        //         msg: "注册成功",
        //         data: Some(list),
        //         total: total.0,
        //     }),
        //     Err(e) => {
        //         eprintln!("数据库查询失败: {:?}", e);
        //         HttpResponse::InternalServerError().body("数据库查询失败")
        //     }
        // }

// pub async fn get_all_users(
//     pool: web::Data<MySqlPool>,
//     query: web::Query<Pagination>,
// ) -> impl Responder {

//     let page = query.page_num.unwrap_or(1);
//     let page_size = query.page_size.unwrap_or(10);
//     let offset = (page - 1) * page_size;

//     // info!("收到查询请求{}",query.page_num.unwrap_or(1));

//     // 查询总数
//     let total: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM sys_user")
//         .fetch_one(pool.get_ref())
//         .await
//     {
//         Ok(count) => count,
//         Err(e) => {
//             eprintln!("查询总数失败: {:?}", e);
//             return HttpResponse::InternalServerError().body("查询总数失败");
//         }
//     };


//     // 分页查询
//     let users = sqlx::query_as::<_, User>("SELECT * FROM sys_user LIMIT ? OFFSET ?")
//         .bind(page_size as i64)
//         .bind(offset as i64)
//         .fetch_all(pool.get_ref())
//         .await;

//     // let users = sqlx::query_as::<_, User>("SELECT * FROM sys_user")
//     //     .fetch_all(pool.get_ref())
//     //     .await;

//     match users {
//         Ok(list) => HttpResponse::Ok().json(
//          ListResponse {
//             code: 200,
//             msg: "注册成功",
//             data: Some(list),
//             total: total.0,
//         }),
//         Err(e) => {
//             eprintln!("数据库查询失败: {:?}", e);
//             HttpResponse::InternalServerError().body("数据库查询失败")
//         }
//     }
// }



    
// pub async fn put_users_update(
//     // 
//     body: web::Json<UpdateAgeRequest>,  // 移除 path，改从 body 获取
// ) -> impl Responder {
//     let updated_user = body.into_inner();
//     let user_id = updated_user.user_id;  // 从 body 获取 user_id
//     info!("更新请求缺少有效字段 age, user_id: {}", user_id);

// avatar
// disease
// name
// password
// phone
// sex
// state
// null
// userId
// user_height
// user_weight
// username
  
//更新用户信息
// pub async fn put_update_users(
//     pool: web::Data<MySqlPool>,
//     path: web::Path<i32>, // user_id
//     body: web::Json<UpdateUserRequest>, // 接收更新的请求体
// ) -> impl Responder{
    
//     let user_id = path.into_inner();
//     let update_request = body.into_inner();
    
//     // 更新用户信息
//     let result = sqlx::query!(
//         r#"
//         UPDATE sys_user
//         SET
//             username = COALESCE(?, username),
//             address = COALESCE(?, address),
//             age = COALESCE(?, age)
//         WHERE user_id = ?"#,
//         update_request.username,
//         update_request.address,
//         update_request.age,
//         user_id
//     )
//     .execute(pool.get_ref())
//     .await;

//     println!("{:?}", result);

//     match result {
//         Ok(res) if res.rows_affected() > 0 => {
//             let response = ApiResponse {
//                 code: 200,
//                 msg: "用户更新成功",
//                 data:  None::<()>,
//             };
//             HttpResponse::Ok().json(response)
//         }
//         Ok(_) => {
//             let response = ApiResponse {
//                 code: 404,
//                 msg: "用户不存在",
//                 data:  None::<()>,
//             };
//             HttpResponse::NotFound().json(response)
//         }
//         Err(e) => {
//             eprintln!("更新用户失败: {:?}", e);
//             let response = ApiResponse {
//                 code: 500,
//                 msg: "更新失败",
//                 data:  None::<()>,
//             };
//             HttpResponse::InternalServerError().json(response)
//         }
//     }
// }



// #[derive(Deserialize)]
// pub struct UpdateUserRequest {
//     pub username: Option<String>,
//     pub address: Option<String>,
//     pub age: Option<i32>,
// }

// #[derive(serde::Serialize)]
// pub struct ApiResponse<T> {
//     pub code: u16,
//     pub msg: String,
//     pub data: T,
// }
// // // 添加用户接口返回值
// #[derive(Deserialize)]
// pub struct UpdateUserRequest {
//     pub username: String, // 用户名
//     pub password: String, // 密码
//     pub address: String,
//     pub age: i32, // 年龄
//     // pub sex: i32,
//     // pub phone: String,
//     // pub disease: String,
//     // pub name: String // 姓名
//     pub sex: i32,
//     pub phone: String,
//     pub disease: String,
//     pub name: String // 姓名
// }
// pub async fn put_update_users(
//     pool: web::Data<MySqlPool>,
//     path: web::Path<i32>, // user_id
//     body: web::Json<UpdateUserRequest>, // 接收更新的请求体
// ) -> impl Responder {
//     let user_id = path.into_inner();
//     let update_request = body.into_inner();

//     // 更新用户信息
//     let result = sqlx::query!(
//         r#"
//         UPDATE sys_user
//         SET 
//             username = COALESCE(?, username),
//             password = COALESCE(?, password),
//             age = COALESCE(?, age),
//             address = COALESCE(?, address),
//             sex = COALESCE(?, sex),
//             phone = COALESCE(?, phone),
//             disease = COALESCE(?, disease),
//             name = COALESCE(?, name)
//         WHERE user_id = ?"#,
//         update_request.username,
//         update_request.password,
//         update_request.age,
//         update_request.address,
//         update_request.sex,
//         update_request.phone,
//         update_request.disease,
//         update_request.name,
//         user_id
//     )
//     .execute(pool.get_ref())
//     .await;

//     println!("{:?}", result); // 输出执行结果

//     match result {
//         Ok(res) if res.rows_affected() > 0 => {
//             let response = ApiResponse {
//                 code: 200,
//                 msg: "用户更新成功",
//                 data: None::<()>,
//             };
//             HttpResponse::Ok().json(response)
//         }
//         Ok(_) => {
//             let response = ApiResponse {
//                 code: 404,
//                 msg: "用户不存在",
//                 data: None::<()>,
//             };
//             HttpResponse::NotFound().json(response)
//         }
//         Err(e) => {
//             eprintln!("更新用户失败: {:?}", e);
//             let response = ApiResponse {
//                 code: 500,
//                 msg: "更新失败",
//                 data: None::<()>,
//             };
//             HttpResponse::InternalServerError().json(response)
//         }
//     }
// }

// // 更新用户
// #[derive(Deserialize, Debug)]
// pub struct UpdateUserRequest {
//     pub username: String, // 用户名
//     pub password: String, // 密码
//     pub user_id: i32, // 用户ID
//     // pub address: String,
//     // pub age: i32, // 年龄
//     // pub sex: i32,
//     // pub phone: String,
//     // pub disease: String,
//     // pub name: String, // 姓名
// }


// use log::{info, error};
// async fn put_update_users(
//     path: web::Path<i32>,              // 用户 ID
//     body: web::Json<UpdateUserRequest>, // 请求体，包含更新的用户数据
// ) -> impl Responder {
//     // 获取路径参数
//     let user_id = path.into_inner();
//     // 直接输出路径参数到命令行
//     println!("Received user_id from path: {}", user_id);

//     // 获取请求体
//     let update_request = body.into_inner();
//     // 直接输出请求体数据到命令行
//     println!("Received update request from body: {:?}", update_request);

//     // 模拟更新逻辑（这里你可以将数据存储到数据库等）
//     // 在实际应用中，这里应该会有数据库操作代码

//     // 返回响应
//     HttpResponse::Ok().json(update_request)  // 返回更新的数据作为响应
// }




// pub async fn put_update_users(
//     pool: web::Data<MySqlPool>,
//     // path: web::Path<i32>, // user_id
//     body: web::Json<UpdateUserRequest>, // 接收更新的请求体
// ) -> impl Responder {
//     // let user_id = body.into_inner();
//      let update_request = body.into_inner();
//     // // 输出请求体的日志
//     // info!("收到更新请求: {:?}", update_request); // 这里使用 `{:?}` 来打印调试信息

//     // let update_request = body.Clone();  // 使用 `.clone()` 来创建副本
//     // let update_request_inner = body.into_inner();  // 这里获取实际数据
//     eprintln!("收到更新请求!");
//     eprintln!("收到更新请求, {:?}", update_request);  

//     // 记录更新请求的日志（避免记录敏感信息）
//     // info!("收到更新请求: 用户ID: {}, 更新内容: username: {}, age: {}, address: {}, sex: {}, phone: {}, disease: {}, name: {}",
//     //     user_id,
//     //     update_request.username,
//     //     update_request.age,
//     //     update_request.address,
//     //     update_request.sex,
//     //     update_request.phone,
//     //     update_request.disease,
//     //     update_request.name
//     // );

//     // 数据验证
//     if !validate_update_request(&update_request) {
//         let response = ApiResponse {
//             code: 400,
//             msg: "请求数据无效",
//             data: None::<()>,
//         };
//         return HttpResponse::BadRequest().json(response);
//     }

//     // update_request.age,
//     // update_request.address,
//     // update_request.sex,
//     // update_request.phone,
//     // update_request.disease,
//     // update_request.name,
//     // age = COALESCE(NULLIF(?, 0), age),
//     // address = COALESCE(NULLIF(?, ''), address),
//     // sex = COALESCE(NULLIF(?, 0), sex),
//     // phone = COALESCE(NULLIF(?, ''), phone),
//     // disease = COALESCE(NULLIF(?, ''), disease),
//     // name = COALESCE(NULLIF(?, ''), name)


//     // 构造更新 SQL 查询
//     let result = sqlx::query!(
//         r#"
//         UPDATE sys_user
//         SET 
//             username = COALESCE(NULLIF(?, ''), username),
//             password = COALESCE(NULLIF(?, ''), password)
//         WHERE user_id = ?
//         "#,
//         update_request.username,
//         update_request.password,
//         update_request.user_id
//     )
//     .execute(pool.get_ref())
//     .await;

//     // 打印日志
//     match result {
//         Ok(res) if res.rows_affected() > 0 => {
//             let response = ApiResponse {
//                 code: 200,
//                 msg: "用户更新成功",
//                 data: None::<()>,
//             };
//             HttpResponse::Ok().json(response)
//         }
//         Ok(_) => {
//             let response = ApiResponse {
//                 code: 404,
//                 msg: "用户不存在",
//                 data: None::<()>,
//             };
//             HttpResponse::NotFound().json(response)
//         }
//         Err(e) => {
//             // 使用日志库记录错误
//             eprintln!("更新用户失败: {:?}", e);
//             let response = ApiResponse {
//                 code: 500,
//                 msg: "更新失败",
//                 data: None::<()>,
//             };
//             HttpResponse::InternalServerError().json(response)
//         }
//     }
// }

// // 数据验证函数 || request.phone.is_empty()
// fn validate_update_request(request: &UpdateUserRequest) -> bool {
//     // 检查字段是否合法
//     if request.username.is_empty() || request.password.is_empty()  {
//         return false;
//     }
//     // if request.age <= 0 || request.sex < 0 || request.sex > 2 { // 假设性别只允许 0, 1, 2
//     //     return false;
//     // }
//     true
// }

 



//  #[derive(Serialize, Deserialize, Debug)]
//  pub struct UpdateUserRequest {
//     username: String,
//     password: String,
//     user_id: i32,
// }


// 打印请求体内容
// info!("收到查询请求: {:?}", body);
// #[derive(Serialize,Debug, Deserialize)]
// pub struct UpdateUserRequest {
//     username: Option<String>,
//     age: Option<i32>,
//     name: Option<String>,
//     sex: Option<String>,
//     address: Option<String>,
//     state: Option<String>,
//     phone: Option<String>,
//     avatar: Option<String>,
//     user_height: Option<f32>,
//     user_weight: Option<f32>,
//     disease: Option<String>,
// }

// pub async fn put_users_update(
//     pool: web::Data<MySqlPool>,
//     path: web::Path<i32>,
//     body: web::Json<UpdateUserRequest>,
// ) -> impl Responder {
//     // let user_id = path.into_inner();
//     let updated_user = body.into_inner();
//     let user_id = path.into_inner();
//     // 检查是否有有效更新字段
//     if !has_valid_fields(&updated_user) {
//         info!("更新请求无有效字段, user_id: {}", user_id);
//         return HttpResponse::BadRequest().json("至少需要一个有效更新字段");
//     }


//     info!("有效字段, user_id: {}", user_id);

//     // 使用 SQLx QueryBuilder 安全构建动态查询
//     let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new("UPDATE sys_user SET ");
//     let mut has_fields = false;

//     // 添加更新字段（使用逗号分隔）
//     let mut separated = query_builder.separated(", ");

//     if let Some(username) = updated_user.username {
//         separated.push("username = ");
//         separated.push_bind(username);
//         has_fields = true;
//     }
//     if let Some(age) = updated_user.age {
//         separated.push("age = ");
//         separated.push_bind(age);
//         has_fields = true;
//     }
//     if let Some(name) = updated_user.name {
//         separated.push("name = ");
//         separated.push_bind(name);
//         has_fields = true;
//     }
//     if let Some(sex) = updated_user.sex {
//         separated.push("sex = ");
//         separated.push_bind(sex);
//         has_fields = true;
//     }
//     if let Some(address) = updated_user.address {
//         separated.push("address = ");
//         separated.push_bind(address);
//         has_fields = true;
//     }
//     if let Some(state) = updated_user.state {
//         separated.push("state = ");
//         separated.push_bind(state);
//         has_fields = true;
//     }
//     if let Some(phone) = updated_user.phone {
//         separated.push("phone = ");
//         separated.push_bind(phone);
//         has_fields = true;
//     }
//     if let Some(avatar) = updated_user.avatar {
//         separated.push("avatar = ");
//         separated.push_bind(avatar);
//         has_fields = true;
//     }
//     if let Some(user_height) = updated_user.user_height {
//         separated.push("user_height = ");
//         separated.push_bind(user_height);
//         has_fields = true;
//     }
//     if let Some(user_weight) = updated_user.user_weight {
//         separated.push("user_weight = ");
//         separated.push_bind(user_weight);
//         has_fields = true;
//     }
//     if let Some(disease) = updated_user.disease {
//         separated.push("disease = ");
//         separated.push_bind(disease);
//         has_fields = true;
//     }

//     // 确保至少有一个有效字段
//     if !has_fields {
//         info!("更新请求无有效字段, user_id: {}", user_id);
//         return HttpResponse::BadRequest().json("至少需要一个有效更新字段");
//     }

//     // 添加 WHERE 条件
//     query_builder.push(" WHERE user_id = ");
//     query_builder.push_bind(user_id);

//     // 构建最终查询
//     let query = query_builder.build();

//     // 执行更新
//     match query.execute(pool.get_ref()).await {
//         Ok(result) => {
//             if result.rows_affected() == 0 {
//                 info!("未找到用户, user_id: {}", user_id);
//                 HttpResponse::NotFound().json("用户不存在")
//             } else {
//                 info!("用户信息更新成功, user_id: {}", user_id);
//                 HttpResponse::Ok().json("更新成功")
//             }
//         }
//         Err(e) => {
//             info!("更新用户信息失败, user_id: {}, 错误: {:?}", user_id, e);
//             HttpResponse::InternalServerError().json("更新失败")
//         }
//     }
// }

// // 检查是否有至少一个有效更新字段
// fn has_valid_fields(user: &UpdateUserRequest) -> bool {
//     user.username.is_some() ||
//     user.age.is_some() ||
//     user.name.is_some() ||
//     user.sex.is_some() ||
//     user.address.is_some() ||
//     user.state.is_some() ||
//     user.phone.is_some() ||
//     user.avatar.is_some() ||
//     user.user_height.is_some() ||
//     user.user_weight.is_some() ||
//     user.disease.is_some()
// }


















//未分离版本
// 查询接口
// 查找用户分页
// pub async fn get_all_users(
//     pool: web::Data<MySqlPool>,
//     query: web::Query<Pagination>,
// ) -> impl Responder {

//     let page = query.page_num.unwrap_or(1);
//     let page_size = query.page_size.unwrap_or(10);
//     let offset = (page - 1) * page_size;

//     // info!("收到查询请求{}",query.page_num.unwrap_or(1));

//     // 查询总数
//     let total: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM sys_user")
//         .fetch_one(pool.get_ref())
//         .await
//     {
//         Ok(count) => count,
//         Err(e) => {
//             eprintln!("查询总数失败: {:?}", e);
//             return HttpResponse::InternalServerError().body("查询总数失败");
//         }
//     };


//     // 分页查询
//     let users = sqlx::query_as::<_, User>("SELECT * FROM sys_user LIMIT ? OFFSET ?")
//         .bind(page_size as i64)
//         .bind(offset as i64)
//         .fetch_all(pool.get_ref())
//         .await;

//     // let users = sqlx::query_as::<_, User>("SELECT * FROM sys_user")
//     //     .fetch_all(pool.get_ref())
//     //     .await;

//     match users {
//         Ok(list) => HttpResponse::Ok().json(
//          ListResponse {
//             code: 200,
//             msg: "注册成功",
//             data: Some(list),
//             total: total.0,
//         }),
//         Err(e) => {
//             eprintln!("数据库查询失败: {:?}", e);
//             HttpResponse::InternalServerError().body("数据库查询失败")
//         }
//     }
// }




// 新增用户
// pub async fn post_add_users(
//     pool: web::Data<MySqlPool>,
//     form: web::Json<AddUserRequest>,
// ) -> HttpResponse {
//     // 1. 检查用户名是否已存在
//     let exists: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM sys_user WHERE username = ?")
//         .bind(&form.username)
//         .fetch_one(pool.get_ref())
//         .await
//     {
//         Ok(count) => count,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ApiResponse {
//                 code: 500,
//                 msg: "数据库错误",
//                 data: None::<()>,
//             })
//         }
//     };
//     if exists.0 > 0 {
//         return HttpResponse::Ok().json(ApiResponse {
//             code: 400,
//             msg: "用户名已存在",
//             data: None::<()>,
//         });
//     }

//     // 2. 密码加密
//     let hashed_pwd = match hash(&form.password, DEFAULT_COST) {
//         Ok(pwd) => pwd,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ApiResponse {
//                 code: 500,
//                 msg: "密码加密失败",
//                 data: None::<()>,
//             })
//         }
//     };

//     // 3. 插入新用户
//     // 插入新用户 
//     let result = sqlx::query(
//         "INSERT INTO sys_user (username,password, age, address, sex, phone,disease,name) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
//     )
   
//     .bind(&form.username)
//     .bind(&hashed_pwd)
//     .bind(&form.age)
//     .bind(&form.address)
//    .bind(form.sex)
//    .bind(&form.phone)
//    .bind(&form.disease)
//    .bind(&form.name)
//     .execute(pool.get_ref())
//     .await;
//     match result {
//         Ok(_) => HttpResponse::Ok().json(ApiResponse {
//             code: 200,
//             msg: "新增用户成功",
//             data: None::<()>,
//         }),
//         Err(e) => {
//             eprintln!("新增用户失败: {:?}", e);
//             HttpResponse::InternalServerError().json(ApiResponse {
//                 code: 500,
//                 msg: "新增用户失败",
//                 data: None::<()>,
//             })
//         }
//     }
// }











// JWT认证中间件
// pub async fn authenticate_token(
//     req: actix_web::HttpRequest,
//     payload: web::Payload,
// ) -> Result<web::ReqData<Claims>, HttpResponse> {
//     // 获取Authorization头
//     let auth_header = req.headers().get("authorization")
//         .ok_or_else(|| HttpResponse::Unauthorized().json(ApiResponse {
//             code: 401,
//             msg: "Token信息缺失",
//             data: None,
//         }))?;
    
//     let auth_str = auth_header.to_str()
//         .map_err(|_| HttpResponse::BadRequest().json(ApiResponse {
//             code: 400,
//             msg: "无效的Token格式",
//             data: None,
//         }))?;
    
//     // 提取Bearer token
//     let token = auth_str.split_whitespace().nth(1)
//         .ok_or_else(|| HttpResponse::Unauthorized().json(ApiResponse {
//             code: 401,
//             msg: "无效的Token格式",
//             data: None,
//         }))?;
    
//     // 验证JWT
//     let key = DecodingKey::from_secret(SECRET_KEY.as_ref());
//     let mut validation = Validation::new(Algorithm::HS256);
//     validation.validate_exp = true;
    
//     let token_data = decode::<Claims>(token, &key, &validation)
//         .map_err(|_| HttpResponse::Forbidden().json(ApiResponse {
//             code: 403,
//             msg: "无效Token",
//             data: None,
//         }))?;
    
//     Ok(web::ReqData::from(token_data))

// }

#[derive(sqlx::FromRow, Serialize, Debug, Clone)]
pub struct MenuTree {
    pub id: i64,                        // 菜单ID
    pub name: String,                   // 菜单名称
    pub path: String,                   // 路由地址
    pub component: Option<String>,      // 组件路径
    pub parent_id: i64,                 // 父菜单ID
    pub sort: i32,                      // 显示顺序
    // pub visible: i32,                  // 是否可见
    pub permission: Option<String>,     // 权限标识
    pub icon: String,                  // 菜单图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuTree>>, // 子菜单
}

// // 扁平化菜单转树形结构
// fn build_menu_tree(menus: Vec<Menu>) -> Vec<MenuTree> {
//     let mut menu_map = std::collections::HashMap::new();
//     let mut root_menus = Vec::new();
    
//     // 将菜单按ID分组
//     for menu in &menus {
//         menu_map.insert(menu.menu_id, MenuTree {
//             id: menu.menu_id,
//             name: menu.menu_name.clone(),
//             path: menu.path.clone(),
//             component: menu.component.clone(),
//             parent_id: menu.parent_id,
//             sort: menu.order_num,
//             // visible: menu.visible == 0,
//             permission: menu.perms.clone(),
//             icon: menu.icon.clone(),
//             children: Some(Vec::new()),
//         });
//     }

//         // 构建父子关系映射
//     let mut parent_child_map = std::collections::HashMap::new();
//     for menu in &menus {
//         parent_child_map.entry(menu.parent_id).or_insert_with(Vec::new).push(menu.menu_id);
//     }
    
//     // 递归构建树形结构
//     fn build_tree(
//         menu_id: i64,
//         menu_map: &std::collections::HashMap<i64, MenuTree>,
//         parent_child_map: &std::collections::HashMap<i64, Vec<i64>>,
//     ) -> MenuTree {
//         let mut menu = menu_map.get(&menu_id).unwrap().clone();
//         if let Some(child_ids) = parent_child_map.get(&menu_id) {
//             menu.children = Some(
//                 child_ids
//                     .iter()
//                     .map(|&id| build_tree(id, menu_map, parent_child_map))
//                     .collect(),
//             );
//         }
//         menu
//     }
    
//     // 构建根菜单
//     for menu in &menus {
//         if menu.parent_id == 0 { // 顶级菜单的parent_id为0
//             root_menus.push(build_tree(menu.menu_id, &menu_map, &parent_child_map));
//         }
//     }

    
//     // 递归排序所有层级的菜单
//     fn sort_menus(menus: &mut Vec<MenuTree>) {
//         menus.sort_by_key(|m| m.sort);
//         if let Some(children) = &mut menus.iter_mut().next().map(|m| &mut m.children) {
//             if let Some(children) = children {
//                 sort_menus(children);
//             }
//         }
//     }
    
//     sort_menus(&mut root_menus);
//     root_menus
// }

