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

// 通过方法
#[allow(unused_imports)]
use crate::common::apimethods::list_api_page; // 引入公共分页查询方法


// 字典类型数据格式
#[allow(unused_imports)]
use crate::common::response::AddDictTypeRequest; // 添加数据格式
#[allow(unused_imports)]
use crate::common::response::DictType; // 基础数据格式
#[allow(unused_imports)]
use crate::common::response::UpdateDictTypeRequest; // 更新数据格式




// 字典数据格式
#[allow(unused_imports)]
use crate::common::response::AddDictDataRequest; // 添加数据格式
#[allow(unused_imports)]
use crate::common::response::DictData; // 基础数据格式
#[allow(unused_imports)]
use crate::common::response::UpdateDictDataRequest; // 更新数据格式

// 测试接口
// pub async fn get_list_all_type() -> HttpResponse {
//     HttpResponse::Ok().json( ApiResponse{
//         code: 200,
//         msg: "接口信息",
//         data:  None::<()>,
//     })
// }

//查询所有字典类型
pub async fn get_all_dict_type(
    pool: web::Data<MySqlPool>,
    query: web::Query<QueryParams>,
    filter: Option<web::Query<HashMap<String, String>>>,
) -> impl Responder {
    // 调试：打印查询参数
    if let Some(ref filter_params) = filter {
        println!("查询参数: {:?}", filter_params);
    }
    // 精确查询字段（exact query）
    let exactquery = vec!["dict_type".to_string()];

    // 模糊查询字段（like query）
    let likequery = vec!["dict_name".to_string()];
    // 调用 list_api_page 传入查询条件
    list_api_page(pool, query, filter, "sys_dict_type", exactquery, likequery).await
}

//查询所有字典数据
pub async fn get_all_dict_data(
    pool: web::Data<MySqlPool>,
    query: web::Query<QueryParams>,
    filter: Option<web::Query<HashMap<String, String>>>,
) -> impl Responder {
    // 调试：打印查询参数
    if let Some(ref filter_params) = filter {
        println!("查询参数: {:?}", filter_params);
    }
    // 精确查询字段（exact query）
    let exactquery = vec!["dict_type".to_string()];

    // 模糊查询字段（like query）
    let likequery = vec!["dict_name".to_string()];
    // 调用 list_api_page 传入查询条件
    list_api_page(pool, query, filter, "sys_dict_data", exactquery, likequery).await
}

//通用查询
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
    let exactquery = vec!["dict_type".to_string()];

    // 模糊查询字段（like query）
    let likequery = vec!["dict_name".to_string()];
    // 调用 list_api_page 传入查询条件
    list_api_page(pool, query, filter, "sys_dict_type", exactquery, likequery).await
}

// 新增
pub async fn post_add(
    pool: web::Data<MySqlPool>,
    form: web::Json<AddDictTypeRequest>,
) -> HttpResponse {
    let mut data = std::collections::HashMap::new();

    // 插入基础字段
    data.insert("dict_name".to_string(), form.dict_name.clone().unwrap_or_default());
    data.insert("dict_type".to_string(), form.dict_type.clone().unwrap_or_default());
    data.insert("status".to_string(), form.status.clone().unwrap_or_default());
    data.insert("remark".to_string(), form.remark.clone().unwrap_or_default());

    // 调用API创建方法
    crate::common::apimethods::create_api(
        pool.get_ref(),
        "sys_dict_type",
        &data,
        &[
            "dict_name".to_string(),
            "dict_type".to_string(),
        ],
    )
    .await
}

// 通用详情
pub async fn get_detail(
    pool: web::Data<MySqlPool>, 
    path: web::Path<i32>
) -> HttpResponse {
    crate::common::apimethods::detail_api::<DictType>(
        pool,
        "sys_dict_type",
        "dict_id",
        path.into_inner(),
    )
    .await
}

// 通用更新
pub async fn put_update(
    pool: web::Data<MySqlPool>,
    form: web::Json<UpdateDictTypeRequest>,
) -> HttpResponse {
    let mut data = HashMap::new();
    // 只更新需要的字段
    // 插入基础字段
    data.insert("dict_name".to_string(), form.dict_name.clone().unwrap_or_default());
    data.insert("dict_type".to_string(), form.dict_type.clone().unwrap_or_default());
    data.insert("status".to_string(), form.status.clone().unwrap_or_default());
    data.insert("remark".to_string(), form.remark.clone().unwrap_or_default());
    crate::common::apimethods::update_api(
        pool.get_ref(),
        "sys_dict_type",
        "dict_id",
        form.dict_id,
        &data,
    )
    .await
}

// 删除
pub async fn del_delete(
    pool: web::Data<MySqlPool>, 
    id: web::Path<i32>
) -> HttpResponse {
    crate::common::apimethods::delete_api(
        pool.get_ref(),
        "sys_dict_type",
        "dict_id",
        *id,
        false, // 软删除，isDeleted=1
    )
    .await
}

// 字典数据
//字典数据查询
pub async fn get_list_data(
    pool: web::Data<MySqlPool>,
    query: web::Query<QueryParams>,
    filter: Option<web::Query<HashMap<String, String>>>,
) -> impl Responder {
    // 调试：打印查询参数
    if let Some(ref filter_params) = filter {
        println!("查询参数: {:?}", filter_params);
    }
    // 精确查询字段（exact query）
    let exactquery = vec![
        "dict_code".to_string(),
        "status".to_string(),
    ];

    // 模糊查询字段（like query）
    let likequery = vec![
        "dict_type".to_string(),
        "dict_label".to_string(),
    ];

    // 调用 list_api_page 传入查询条件
    list_api_page(pool, query, filter, "sys_dict_data", exactquery, likequery).await
}

// 新增
pub async fn post_add_data(
    pool: web::Data<MySqlPool>,
    form: web::Json<AddDictDataRequest>,
) -> HttpResponse {
    let mut data = std::collections::HashMap::new();
    // 添加相关字段
//    data.insert("dict_type".to_string(), form.dict_type.clone().unwrap_or_default());
//     data.insert("dict_label".to_string(), form.dict_label.clone().unwrap_or_default());
//     data.insert("dict_value".to_string(), form.dict_value.clone().unwrap_or_default());
//     data.insert("dict_sort".to_string(), form.dict_sort.map(|v| v.to_string()).unwrap_or_default());
//     data.insert("remark".to_string(), form.remark.clone().unwrap_or_default());
//     data.insert("status".to_string(), form.status.clone().unwrap_or_default());

    // 调用API创建方法
    crate::common::apimethods::create_api(
        pool.get_ref(),
        "sys_dict_data",
        &data,
        &[], //"menu_name".to_string()
    )
    .await
}

// 字典数据详情
pub async fn get_detail_data(pool: web::Data<MySqlPool>, path: web::Path<i32>) -> HttpResponse {
    crate::common::apimethods::detail_api::<DictData>(
        pool,
        "sys_dict_data",
        "dict_data_id",
        path.into_inner(),
    )
    .await
}
// 字典数据更新
pub async fn put_update_data(
    pool: web::Data<MySqlPool>,
    form: web::Json<UpdateDictDataRequest>,
) -> HttpResponse {
    let mut data = HashMap::new();
    // 只更新需要的字段
    data.insert("dict_type".to_string(), form.dict_type.clone().unwrap_or_default());
    data.insert("dict_label".to_string(), form.dict_label.clone().unwrap_or_default());
    data.insert("dict_value".to_string(), form.dict_value.clone().unwrap_or_default());
    data.insert("dict_sort".to_string(), form.dict_sort.map(|v| v.to_string()).unwrap_or_default());
    data.insert("remark".to_string(), form.remark.clone().unwrap_or_default());
    data.insert("status".to_string(), form.status.clone().unwrap_or_default());
    crate::common::apimethods::update_api(
        pool.get_ref(),
        "sys_dict_data",
        "dict_data_id",
        form.dict_data_id,
        &data,
    )
    .await
}
// 字典数据删除
pub async fn del_delete_data(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> HttpResponse {
    crate::common::apimethods::delete_api(
        pool.get_ref(),
        "sys_dict_data",
        "dict_data_id",
        *id,
        false, // 软删除，isDeleted=1
    )
    .await
}
