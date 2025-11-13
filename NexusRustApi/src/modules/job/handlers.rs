use actix_web::{web,HttpResponse, Responder};
// , HttpRequest
use sqlx::{MySqlPool};
// ,MySql, Pool

#[allow(unused_imports)]
use crate::common::response::ApiResponse;// 导入 ApiResponse 模型

// 返回类型
#[allow(unused_imports)]
use crate::common::response::ListQueryParams;// 分页接口返回值

#[allow(unused_imports)]
use crate::common::response::QueryParams;// 查询参数

#[allow(unused_imports)]
use std::collections::HashMap; //模糊查询



// 通过方法
#[allow(unused_imports)]
use crate::common::apimethods::list_api_page; // 引入公共分页查询方法

#[allow(unused_imports)]
use crate::common::response::Job; // 导入Job模型

 // 新增结构体
#[allow(unused_imports)]
use crate::common::response::AddJobRequest;

// 更新结构体
#[allow(unused_imports)]
use crate::common::response::UpdateJobRequest; // 导入Job模型


// 测试接口
// pub async fn get_list() -> HttpResponse {
//     HttpResponse::Ok().json(ApiResponse {
//         code: 200,
//         msg: "接口信息",
//         data:  None::<()>,
//     })
// }
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
    // 模糊查询字段（like query） "Job_name".to_string()
    let likequery = vec![
        "Job_name".to_string(),
    ];

   // 调用 list_api_page 传入查询条件
   list_api_page(pool, query, filter, "jobs", exactquery, likequery).await
}

// 新增
pub async fn post_add(
    pool: web::Data<MySqlPool>,
    form: web::Json<AddJobRequest>,
) -> HttpResponse {
    let mut data = std::collections::HashMap::new();
    println!("FROM新增: {:?}", form);

    data.insert("job_title".to_string(), form.job_title.clone()); // 标题
    data.insert("department".to_string(), form.department.clone()); // 部门
    data.insert("job_requirements".to_string(), form.job_requirements.clone().unwrap_or_default()); // 工作要求
    data.insert("job_responsibilities".to_string(), form.job_responsibilities.clone().unwrap_or_default()); // 工作职责
    data.insert("job_status".to_string(), form.job_status.clone()); // 职位状态
    data.insert("job_type".to_string(), form.job_type.clone()); // 职位类型
    data.insert("location".to_string(), form.location.clone()); // 工作地点
    data.insert("salary_range".to_string(), form.salary_range.clone()); // 薪资范围
    crate::common::apimethods::create_api(
        pool.get_ref(),
        "jobs",
        &data,
        &[ 
            // "Job_name".to_string(),
            // "Job_key".to_string()
        ],
    ).await
}

// 通用详情
pub async fn get_detail(
    pool: web::Data<MySqlPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    crate::common::apimethods::detail_api::<Job>(pool, "sys_Job", "Job_id", path.into_inner()).await
}

// 通用更新
pub async fn put_update(
    pool: web::Data<MySqlPool>, 
    form: web::Json<UpdateJobRequest>
) -> HttpResponse {
    let mut data = HashMap::new();
    println!("FROM更新: {:?}", form);
    
    data.insert("job_title".to_string(), form.job_title.clone()); // 标题
    data.insert("department".to_string(), form.department.clone()); // 部门
    data.insert("job_requirements".to_string(), form.job_requirements.clone().unwrap_or_default()); // 工作要求
    data.insert("job_responsibilities".to_string(), form.job_responsibilities.clone().unwrap_or_default()); // 工作职责
    data.insert("job_status".to_string(), form.job_status.clone()); // 职位状态
    data.insert("job_type".to_string(), form.job_type.clone()); // 职位类型
    data.insert("location".to_string(), form.location.clone()); // 工作地点
    data.insert("salary_range".to_string(), form.salary_range.clone()); // 薪资范围
    
    crate::common::apimethods::update_api(
        pool.get_ref(),
        "Jobs",
        "job_id",
        form.job_id,
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
        "sys_Job",
        "Job_id",
        *id,
        false, // 软删除，isDeleted=1
    ).await
}