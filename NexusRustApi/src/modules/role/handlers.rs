use actix_web::{web, HttpResponse, Responder}; //, HttpRequest
use sqlx::{MySqlPool}; //,MySql, Pool
// use crate::common::response::ApiResponse;// 导入 ApiResponse 模型

// 返回类型
#[allow(unused_imports)]
use crate::common::response::ListQueryParams;// 分页接口返回值

#[allow(unused_imports)]
use crate::common::response::QueryParams;// 查询参数

#[allow(unused_imports)]
use std::collections::HashMap; //模糊查询

#[allow(unused_imports)]
use crate::common::response::AddRoleRequest; // 角色请求结构体

// 通过方法
#[allow(unused_imports)]
use crate::common::apimethods::list_api_page; // 引入公共分页查询方法

// 角色
#[allow(unused_imports)]
use crate::common::response::Role; // 导入Role模型

#[allow(unused_imports)]
use crate::common::response::UpdateRoleRequest; // 导入Role模型


// 通用查询
pub async fn get_list(
    pool: web::Data<MySqlPool>,
    query: web::Query<QueryParams>,
    filter: Option<web::Query<HashMap<String, String>>>
) -> impl Responder {
    // 调试：打印查询参数
    if let Some(ref filter_params) = filter {
        println!("查询参数: {:?}", filter_params);
    }
    // 精确查询字段（exact query）
    let exactquery = vec![
        "status".to_string(),
    ];
    // 模糊查询字段（like query） "role_name".to_string()
    let likequery = vec![
        "role_name".to_string(),
    ];

    // // 处理字段名映射：将 roleName 映射到 role_name
    // let mut filter_map = filter.map(|f| f.into_inner()).unwrap_or_default();
    
    // if let Some(role_name) = filter_map.remove("roleName") {
    //     filter_map.insert("role_name".to_string(), role_name);
    // }

   // 调用 list_api_page 传入查询条件
   list_api_page(pool, query, filter, "sys_role", exactquery, likequery).await
}

// 新增
pub async fn post_add(
    pool: web::Data<MySqlPool>,
    form: web::Json<AddRoleRequest>,
) -> HttpResponse {
    let mut data = std::collections::HashMap::new();
    data.insert("role_name".to_string(), form.role_name.clone());
    data.insert("role_key".to_string(), form.role_key.clone());
    data.insert("role_sort".to_string(), form.role_sort.to_string());
    data.insert("status".to_string(), form.status.clone());
    data.insert("remark".to_string(), form.remark.clone());

     // 处理布尔值字段
     data.insert("menu_check_strictly".to_string(), if form.menu_check_strictly { "1" } else { "0" }.to_string());
     data.insert("dept_check_strictly".to_string(), if form.dept_check_strictly { "1" } else { "0" }.to_string());

    crate::common::apimethods::create_api(
        pool.get_ref(),
        "sys_role",
        &data,
        &[ 
            "role_name".to_string(),
            "role_key".to_string()
        ],
    ).await
}

// 通用详情
pub async fn get_detail(
    pool: web::Data<MySqlPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    crate::common::apimethods::detail_api::<Role>(pool, "sys_role", "role_id", path.into_inner()).await
}

// 通用更新
pub async fn put_update(
    pool: web::Data<MySqlPool>, 
    item: web::Json<UpdateRoleRequest>
) -> HttpResponse {
    let mut data = HashMap::new();
    // 只更新需要的字段
    data.insert("status".to_string(), item.status.clone());
    data.insert("role_name".to_string(), item.role_name.clone());
    data.insert("remark".to_string(), item.remark.clone());
    crate::common::apimethods::update_api(
        pool.get_ref(),
        "sys_role",
        "role_id",
        item.role_id,
        &data,
    ).await
}

// 通用真删除
pub async fn del_delete(
    pool: web::Data<MySqlPool>,
    id: web::Path<i32>
) -> HttpResponse {
    crate::common::apimethods::delete_api(
        pool.get_ref(),
        "sys_role",
        "role_id",
        *id,
        false, // 软删除，isDeleted=1
    ).await
}