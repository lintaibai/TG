use actix_web::{HttpResponse, Responder, web}; //, HttpRequest
use sqlx::MySqlPool; //,MySql, Pool

#[allow(unused_imports)]
use crate::common::response::ApiResponse; // 导入 ApiResponse 模型

// 返回类型
#[allow(unused_imports)]
use crate::common::response::ListQueryParams; // 分页接口返回值

#[allow(unused_imports)]
use crate::common::response::QueryParams; // 查询参数

#[allow(unused_imports)]
use std::collections::HashMap; //模糊查询

#[allow(unused_imports)]
use crate::common::response::AddMenuRequest; // 新增结构体

// 通过方法
#[allow(unused_imports)]
use crate::common::apimethods::list_api_page; // 引入公共分页查询方法

// 菜单
#[allow(unused_imports)]
use crate::common::response::Menu; // 导入Menu模型

#[allow(unused_imports)]
use crate::common::response::UpdateMenuRequest; // 导入Menu模型

// 通用查询
pub async fn get_list(
    pool: web::Data<MySqlPool>,
    query: web::Query<QueryParams>,
    filter: Option<web::Query<HashMap<String, String>>>,
) -> impl Responder {
    // 调试：打印查询参数
    if let Some(ref filter_params) = filter {
        println!("查询参数: {:?}", filter_params);
    }
    // 精确查询字段（exact query）
    let exactquery = vec!["status".to_string()];
    // 模糊查询字段（like query） "role_name".to_string()
    let likequery = vec!["menu_name".to_string()];
    // 调用 list_api_page 传入查询条件
    list_api_page(pool, query, filter, "sys_menu", exactquery, likequery).await
}

// 新增
pub async fn post_add(pool: web::Data<MySqlPool>, form: web::Json<AddMenuRequest>) -> HttpResponse {
    let mut data = std::collections::HashMap::new();
    
    // 插入基础字段
    data.insert("parent_id".to_string(), form.parent_id.to_string());
    data.insert("menu_name".to_string(), form.menu_name.clone());
    data.insert("menu_type".to_string(), form.menu_type.clone());
    data.insert("order_num".to_string(), form.order_num.to_string());
    data.insert("path".to_string(), form.path.clone());
    data.insert("status".to_string(), form.status.clone());

    // 将布尔值转换为字符串格式
    data.insert("is_frame".to_string(), form.is_frame.clone());
    data.insert("is_cache".to_string(), form.is_cache.clone());
    data.insert("visible".to_string(), form.visible.clone());

    // 调用API创建方法
    crate::common::apimethods::create_api(
        pool.get_ref(),
        "sys_menu",
        &data,
        &[
            "menu_name".to_string(),
            "path".to_string(),
        ],
    )
    .await
}

// 通用详情
pub async fn get_detail(
    pool: web::Data<MySqlPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    crate::common::apimethods::detail_api::<Menu>(pool, "sys_menu", "menu_id", path.into_inner()).await
}

// 通用更新
pub async fn put_update(
    pool: web::Data<MySqlPool>,
    item: web::Json<UpdateMenuRequest>,
) -> HttpResponse {
    let mut data = HashMap::new();
    // 只更新需要的字段
    // 插入基础字段
    data.insert("parent_id".to_string(), item.parent_id.to_string());
    data.insert("menu_name".to_string(), item.menu_name.clone());
    data.insert("menu_type".to_string(), item.menu_type.clone());
    data.insert("order_num".to_string(), item.order_num.to_string());
    data.insert("path".to_string(), item.path.clone());
    data.insert("status".to_string(), item.status.clone());
    data.insert("is_frame".to_string(), item.is_frame.to_string());
    data.insert("is_cache".to_string(), item.is_cache.to_string());
    data.insert("visible".to_string(), item.visible.clone());
    if let Some(component) = item.component.as_ref() {
        data.insert("component".to_string(), component.clone());
    }
    crate::common::apimethods::update_api(
        pool.get_ref(),
        "sys_menu",
        "menu_id",
        item.menu_id,
        &data,
    )
    .await
}

// 删除
pub async fn del_delete(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> HttpResponse {
    crate::common::apimethods::delete_api(
        pool.get_ref(),
        "sys_menu",
        "menu_id",
        *id,
        false, // 软删除，isDeleted=1
    )
    .await
}
