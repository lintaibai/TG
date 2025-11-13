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

// 当前模块部分
// 结构体
#[allow(unused_imports)]
use crate::common::response::AddDept; 

// 菜单
#[allow(unused_imports)]
use crate::common::response::Dept; // 导入Menu模型

#[allow(unused_imports)]
use crate::common::response::UpdateMenuRequest; // 导入Menu模型

// 测试接口
// pub async fn get_list() -> HttpResponse {
//     HttpResponse::Ok().json( ApiResponse{
//         code: 200,
//         msg: "接口信息",
//         data:  None::<()>,
//     })
// }

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
    let likequery = vec!["dept_name".to_string()];
    // 调用 list_api_page 传入查询条件
    list_api_page(pool, query, filter, "sys_dept", exactquery, likequery).await
}
// 新增
pub async fn post_add(pool: web::Data<MySqlPool>, form: web::Json<AddDept>) -> HttpResponse {
    let mut data = std::collections::HashMap::new();
    // 插入基础字段
    data.insert("parent_id".to_string(), form.parent_id.clone());
    data.insert("dept_name".to_string(), form.dept_name.clone());
    data.insert("order_num".to_string(), form.order_num.to_string());
    data.insert("leader".to_string(), form.leader.clone());
    data.insert("phone".to_string(), form.phone.clone());
    data.insert("email".to_string(), form.email.clone());
    data.insert("status".to_string(), form.status.clone());
    // 调用API创建方法
    crate::common::apimethods::create_api(
        pool.get_ref(),
        "sys_dept",
        &data,
        &[
            "dept_name".to_string(),
        ],
    )
    .await
}

// 通用详情
pub async fn get_detail(
    pool: web::Data<MySqlPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    crate::common::apimethods::detail_api::<Dept>(pool, "sys_dept", "dept_id", path.into_inner()).await
}

// 通用更新
pub async fn put_update(
    pool: web::Data<MySqlPool>,
    form: web::Json<Dept>,
) -> HttpResponse {
    let mut data = HashMap::new();
    // 插入字段
   
   // 插入基础字段
    data.insert("parent_id".to_string(), form.parent_id.to_string());
    data.insert("dept_name".to_string(), form.dept_name.clone());
    data.insert("order_num".to_string(), form.order_num.to_string());
    data.insert("leader".to_string(), form.leader.clone());
    data.insert("phone".to_string(), form.phone.clone());
    data.insert("email".to_string(), form.email.clone());
    data.insert("status".to_string(), form.status.clone());

    crate::common::apimethods::update_api(
        pool.get_ref(),
        "sys_dept",
        "dept_id",
        form.dept_id as i32,
        &data,
    )
    .await
}

// 删除
pub async fn del_delete(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> HttpResponse {
    crate::common::apimethods::delete_api(
        pool.get_ref(),
        "sys_dept",
        "dept_id",
        *id,
        false, // 软删除，isDeleted=1
    )
    .await
}


// 流式导出csv数据
// CSV 转义
fn escape_csv(s: &str) -> String {
    let need_quote = s.contains(['"', ',', '\n', '\r']);
    if need_quote {
        let escaped = s.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        s.to_string()
    }
}
// 流式导出 CSV
pub async fn post_export(pool: web::Data<MySqlPool>) -> HttpResponse {
    use actix_web::web::Bytes;
    use futures_util::stream::{self, StreamExt};

    // BOM + 表头
    let head = {
        let mut v = vec![0xEF, 0xBB, 0xBF];
        v.extend_from_slice("部门ID,父部门ID,部门名称,显示顺序,负责人,联系电话,邮箱,状态\n".as_bytes());
        v
    };
    let head_stream = stream::once(async { Ok::<Bytes, actix_web::Error>(Bytes::from(head)) });

    // 分页流：每次查询一批，转换为 Bytes 后产出
    let pool = pool.get_ref().clone();
    let rows_stream = stream::unfold(Some((pool, 0i64)), |state| async move {
        let (pool, last_id) = state?;

        let rows = match sqlx::query_as::<_, Dept>(
            "SELECT dept_id, parent_id, dept_name, order_num, leader, phone, email, status
             FROM sys_dept
             WHERE del_flag = 0 AND dept_id > ?
             ORDER BY dept_id
             LIMIT 1000"
        )
        .bind(last_id)
        .fetch_all(&pool)
        .await
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("export_users query error: {e:?}");
                return Some((Err(actix_web::error::ErrorInternalServerError("查询数据失败")), None));
            }
        };

        if rows.is_empty() {
            return None;
        }

        let mut next_last = last_id;
        let mut buf = String::with_capacity(rows.len() * 128);
        for u in rows {
            next_last = u.dept_id.max(next_last);
            let line = format!(
                "{},{},{},{},{},{},{},{}\n",
                u.dept_id,
                u.parent_id,
                escape_csv(&u.dept_name),
                u.order_num,
                escape_csv(&u.leader),
                escape_csv(&u.phone),
                escape_csv(&u.email),
                u.status,
               
            );
            buf.push_str(&line);
        }

        Some((Ok(Bytes::from(buf)), Some((pool, next_last))))
    });

    HttpResponse::Ok()
        .content_type("application/x-www-form-urlencoded; charset=utf-8")
        .append_header(("Content-Disposition", "attachment; filename=\"users.csv\""))
        .append_header(("Access-Control-Expose-Headers", "Content-Disposition, Content-Type"))
        .streaming(head_stream.chain(rows_stream))
}