use actix_web::{HttpResponse, Responder, web}; //, HttpRequest
use sqlx::MySqlPool; //,MySql, Pool

// 模糊查询
#[allow(unused_imports)]
use std::collections::HashMap;

// 导入ApiResponse 模型
#[allow(unused_imports)]
use crate::common::response::ApiResponse;

// 导入分页接口返回类型
#[allow(unused_imports)]
use crate::common::response::ListQueryParams; // 分页接口返回值

// 导入查询参数
#[allow(unused_imports)]
use crate::common::response::QueryParams;

// 引入公共分页查询方法
#[allow(unused_imports)]
use crate::common::apimethods::list_api_page;

// 当前模块部分结构体
#[allow(unused_imports)]
use crate::common::response::Notice;
// 添加
#[allow(unused_imports)]
use crate::common::response::AddNotice; // 导入Notice模型
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
    let likequery = vec!["notice_title".to_string()];
    // 调用 list_api_page 传入查询条件
    list_api_page(pool, query, filter, "sys_notice", exactquery, likequery).await
}
// 新增
pub async fn post_add(pool: web::Data<MySqlPool>, form: web::Json<AddNotice>) -> HttpResponse {
    let mut data = std::collections::HashMap::new();
    // 插入基础字段
    data.insert("notice_title".to_string(), form.base.notice_title.clone());
    data.insert("notice_type".to_string(), form.base.notice_type.clone());
    data.insert("notice_content".to_string(), form.base.notice_content.clone());
    data.insert("status".to_string(), form.base.status.clone());
    // 调用API创建方法
    crate::common::apimethods::create_api(pool.get_ref(), "sys_notice", &data, &[]).await
}

// 通用详情
pub async fn get_detail(pool: web::Data<MySqlPool>, path: web::Path<i32>) -> HttpResponse {
    crate::common::apimethods::detail_api::<Notice>(pool, "sys_notice", "notice_id", path.into_inner())
        .await
}

// 通用更新
pub async fn put_update(pool: web::Data<MySqlPool>, form: web::Json<Notice>) -> HttpResponse {
    let mut data = HashMap::new();
    // 插入字段

    // 插入基础字段
    data.insert("notice_title".to_string(), form.notice_title.clone());
    data.insert("notice_type".to_string(), form.notice_type.clone());
    data.insert("notice_content".to_string(), form.notice_content.clone());
    data.insert("status".to_string(), form.status.clone());

    crate::common::apimethods::update_api(
        pool.get_ref(),
        "sys_notice",
        "notice_id",
        form.notice_id.unwrap(),
        &data,
    )
    .await
}

// 删除
pub async fn del_delete(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> HttpResponse {
    crate::common::apimethods::delete_api(
        pool.get_ref(),
        "sys_notice",
        "notice_id",
        *id,
        false, // 软删除，isDeleted=1
    )
    .await
}