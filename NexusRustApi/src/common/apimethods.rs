use bcrypt::{hash, DEFAULT_COST};
use sqlx::FromRow;

#[allow(unused_imports)]
use actix_web::{HttpRequest, HttpResponse, Responder, web};

#[allow(unused_imports)]
use sqlx::{Column, Error, MySql, MySqlPool, Pool, Row};

#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::common::response::Pagination; // 分页接口返回值

#[allow(unused_imports)]
use crate::common::response::QueryParams; // 分页接口返回值


#[allow(unused_imports)]
use crate::common::response::ListResponse; //列表接口返回值

#[allow(unused_imports)]
use crate::modules::user::models::User; // 导入 User 模型

#[allow(unused_imports)]
use crate::common::response::ListQuery; // 导入查询模型

// 返回信息

#[allow(unused_imports)]
use crate::common::response::BasicResponse; //接口返回值

use crate::common::response::ApiDetailResponse;// 详情接口返回值


#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// 添加一个字段映射函数
fn map_field_name(column_name: &str) -> String {
    // 首先检查是否有预定义的映射
    match column_name {
        "user_id" => "userId".to_string(),
        "user_height" => "userHeight".to_string(),
        "user_weight" => "userWeight".to_string(),
        _ => {
            // 如果没有预定义映射，则自动转换 snake_case 为 camelCase
            snake_to_camel_case(column_name)
        }
    }
}

// 高级 snake_case 转 camelCase 函数
// 正确的 snake_case 转 camelCase 函数
fn snake_to_camel_case(snake: &str) -> String {
    if snake.is_empty() {
        return String::new();
    }
    
    let mut result = String::new();
    let mut capitalize_next = false;
    let mut first_char = true;
    let mut prev_was_underscore = false;
    
    for ch in snake.chars() {
        if ch == '_' {
            if !prev_was_underscore {
                capitalize_next = true;
            }
            prev_was_underscore = true;
        } else {
            if capitalize_next {
                // 下划线后的字符大写
                result.push(ch.to_ascii_uppercase());
                capitalize_next = false;
            } else if first_char {
                // 第一个字符小写
                result.push(ch.to_ascii_lowercase());
                first_char = false;
            } else {
                // 其他字符保持原样
                result.push(ch);
            }
            prev_was_underscore = false;
        }
    }
    
    result
}

// 驼峰命名转下划线命名 roleName => role_name
fn camel_to_snake (field: &str) -> String {
    // 特殊映射
    match field {
        "pageNum" => "page_num".to_string(),
        "pageSize" => "page_size".to_string(),
        _ => {
           // 通用驼峰命名转下划线命名
           let mut result = String::new();
           let mut chars = field.chars().peekable();
           
           while let Some(ch) = chars.next() {
               if ch.is_uppercase() {
                   if !result.is_empty() {
                       result.push('_');
                   }
                   result.push(ch.to_lowercase().next().unwrap());
               } else {
                   result.push(ch);
               }
           }
           result
        }
    }
}



#[allow(dead_code)]
// 公共查询方法
pub async fn list_api(
    pool: &Pool<MySql>,
    query: Option<&str>,
) -> Result<ApiResponse<Vec<HashMap<String, String>>>, Error> {
    // 如果没有传入查询语句，直接返回空数据
    if query.is_none() {
        return Ok(ApiResponse {
            code: 200,
            message: "请求成功".to_string(),
            data: Some(vec![]), // 返回空数据
        });
    }

    // 执行传入的查询语句
    let query = query.unwrap(); // 如果传入查询语句

    // 输出日志帮助调试（可选）
    println!("执行的查询: {}", query);

    // 使用 SQLx 提供的 `query` 来执行查询，不进行结构体映射
    let rows = sqlx::query(query).fetch_all(pool).await?;

    // 处理查询结果，并将每一行的数据映射到 HashMap
    let mut records = Vec::new();
    for row in rows {
        // 每一行是一个 HashMap，键为列名，值为列的值
        let mut record = HashMap::new();

        // 动态地遍历当前行的所有列
        for (i, _column) in row.columns().iter().enumerate() {
            // 使用列的索引（i）来获取列的名称
            let column_name = row.columns().get(i).unwrap().name().to_string();

            // 使用 try_get 获取每个列的值，可以适配不同的类型
            let value: String = row.try_get(i)?;

            // 将列名和对应的值存储到 HashMap
            record.insert(column_name, value);
        }

        // 将每一行的 HashMap 添加到记录中
        records.push(record);
        println!("结果: {:?}", records);
    }

    // 返回带有数据的成功响应
    Ok(ApiResponse {
        code: 200,
        message: "请求成功".to_string(),
        data: Some(records),
    })
}


#[allow(dead_code)]
pub async fn list_api_page(
    pool: web::Data<MySqlPool>,
    query: web::Query<QueryParams>,
    _filter: Option<web::Query<HashMap<String, String>>>, // 动态查询条件
    table_name: &str,  // 表名
    exactquery: Vec<String>,  // 精确查询字段名
    likequery: Vec<String>,   // 模糊查询字段名
) -> impl Responder {
    
    // 查询总数的 SQL
    let mut query_total = format!("SELECT COUNT(*) FROM {}", table_name);

    // 查询条件
    let mut query_params: Vec<String> = Vec::new();  // 存储查询参数
    let mut where_clauses = Vec::new();

    // 统一归一化 filter 的 key（驼峰转下划线）
    let normalized_filter: HashMap<String, String> = _filter
        .as_ref()
        .map(|f| {
            f.iter()
             .map(|(k, v)| (camel_to_snake(k), v.clone())) // 统一转下划线
             .collect()
        })
        .unwrap_or_default();

    // 调试输出
    if !normalized_filter.is_empty() {
        println!("归一化后的查询参数: {:?}", normalized_filter);
    }


    // 处理精确查询（exactquery）
    // for field in exactquery {
    //     if let Some(value) = _filter.as_ref().and_then(|f| f.get(&field)) {
    //         if !value.is_empty() { // 确保值不为空字符串
    //             where_clauses.push(format!("{} = ?", field));  // 精确查询
    //             query_params.push(value.clone());  // 将非空值加入查询条件
    //         }
    //     }
    // }
   
    // 处理模糊查询（likequery）
    // for field in likequery {
    //     if let Some(value) = _filter.as_ref().and_then(|f| f.get(&field)) {
    //         if !value.is_empty() { // 确保值不为空字符串
    //             where_clauses.push(format!("{} LIKE ?", field));  // 模糊查询
    //             query_params.push(format!("%{}%", value));  // 模糊查询时需要加上 %
    //         }
    //     }
    // }


    // 处理精确查询（exactquery）
    // for field in exactquery {
        //     // 检查原始字段名和映射后的字段名
        //     let mut found_value = None;
            
        //     // 先检查原始字段名
        //     if let Some(value) = _filter.as_ref().and_then(|f| f.get(&field)) {
        //         if !value.is_empty() {
        //             found_value = Some(value.clone());
        //         }
        //     }
            
        //     // 如果没找到，检查映射后的字段名
        //     if found_value.is_none() {
        //         let mapped_field = camel_to_snake(&field);
        //         if mapped_field != field {
        //             if let Some(value) = _filter.as_ref().and_then(|f| f.get(&mapped_field)) {
        //                 if !value.is_empty() {
        //                     found_value = Some(value.clone());
        //                 }
        //             }
        //         }
        //     }
            
        //     // 如果找到了值，添加到查询条件
        //     if let Some(value) = found_value {
        //         where_clauses.push(format!("{} = ?", field));
        //         query_params.push(value);
        //     }
        // }
        // 处理模糊查询（likequery）
        // for field in likequery {
        //     let mut found_value = None;
            
        //     // 先检查原始字段名
        //     if let Some(value) = _filter.as_ref().and_then(|f| f.get(&field)) {
        //         if !value.is_empty() {
        //             found_value = Some(value.clone());
        //         }
        //     }
            
        //     // 如果没找到，检查映射后的字段名
        //     if found_value.is_none() {
        //         let mapped_field = camel_to_snake(&field);
        //         if mapped_field != field {
        //             if let Some(value) = _filter.as_ref().and_then(|f| f.get(&mapped_field)) {
        //                 if !value.is_empty() {
        //                     found_value = Some(value.clone());
        //                 }
        //             }
        //         }
        //     }
            
        //     // 如果找到了值，添加到查询条件
        //     if let Some(value) = found_value {
        //         where_clauses.push(format!("{} LIKE ?", field));
        //         query_params.push(format!("%{}%", value));
        //     }
        // }


      // 处理精确查询（exactquery）
      for field in exactquery {
        if let Some(value) = normalized_filter.get(&field) {
            if !value.is_empty() {
                where_clauses.push(format!("{} = ?", field));
                query_params.push(value.clone());
            }
        }
    }
    // 处理模糊查询（likequery）
    for field in likequery {
        if let Some(value) = normalized_filter.get(&field) {
            if !value.is_empty() {
                where_clauses.push(format!("{} LIKE ?", field));
                query_params.push(format!("%{}%", value));
            }
        }
    }


    // 处理分页参数，避免空字符串、无效字符串导致解析错误
    let mut page_num = None;
    let mut page_size = None;
    
    // 判断 `page_num` 是否有效，只有有效时才赋值
    if let Some(num_str) = &query.page_num {
        if !num_str.is_empty() {
            page_num = num_str.parse::<u32>().ok(); // 使用 `ok()` 来忽略解析失败的情况
        }
    }
    
    // 判断 `page_size` 是否有效，只有有效时才赋值
    if let Some(size_str) = &query.page_size {
        if !size_str.is_empty() {
            page_size = size_str.parse::<u32>().ok(); // 使用 `ok()` 来忽略解析失败的情况
        }
    }

    // 计算分页偏移量，如果没有有效分页参数，则不应用分页
    let offset = match (page_num, page_size) {
        (Some(num), Some(size)) => (num - 1) * size,
        _ => 0, // 如果没有有效的分页参数，则不添加分页
    };

    // 如果有查询条件，添加 WHERE 子句
    if !where_clauses.is_empty() {
        query_total.push_str(" WHERE ");
        query_total.push_str(&where_clauses.join(" AND "));
    }

    // 创建查询总数
    let mut query = sqlx::query_as::<_, (i64,)>(&query_total);

    // 遍历并逐个绑定参数
    for value in query_params.iter() {
        query = query.bind(value);  // 绑定查询参数
    }

    let total: (i64,) = match query.fetch_one(pool.get_ref()).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("查询总数失败: {:?}", e);
            return HttpResponse::InternalServerError().body("查询总数失败");
        }
    };

    // 分页查询 SQL
    let mut query = format!("SELECT * FROM {}", table_name);

    // 如果有查询条件，拼接 WHERE 子句
    if !where_clauses.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&where_clauses.join(" AND "));
    }

    // 仅在有有效分页参数时，添加 LIMIT 和 OFFSET
    if let (Some(page_size), Some(_page_num)) = (page_size, page_num) {
        query.push_str(" LIMIT ? OFFSET ?");
        query_params.push(page_size.to_string());  // 添加分页参数
        query_params.push(offset.to_string());    // 添加偏移量
    }

    // 创建查询
    // let mut query = sqlx::query_as::<_, User>(&query);
    // for value in query_params.iter() {
    //     query = query.bind(value);  // 绑定分页参数
    // }

    // 执行查询
    // let resultlist = match query.fetch_all(pool.get_ref()).await {
    //     Ok(list) => list,
    //     Err(e) => {
    //         eprintln!("数据库查询失败: {:?}", e);
    //         return HttpResponse::InternalServerError().body("数据库查询失败");
    //     }
    // };

    // 使用 sqlx::query 而不是 query_as，返回原始行数据
    let mut query = sqlx::query(&query);
    for value in query_params.iter() {
        query = query.bind(value);
    }

    // 执行查询并处理结果
    let rows = match query.fetch_all(pool.get_ref()).await {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("数据库查询失败: {:?}", e);
            return HttpResponse::InternalServerError().body("数据库查询失败");
        }
    };

    // 将行数据转换为 HashMap
    let mut resultlist = Vec::new();
    for row in rows {
        let mut record = HashMap::new();
        for (i, column) in row.columns().iter().enumerate() {
            let column_name = column.name().to_string();
            let mapped_name = map_field_name(&column_name);

            // 尝试获取不同类型的值
            if let Ok(value) = row.try_get::<String, _>(i) {
                record.insert(mapped_name, value);
            } else if let Ok(value) = row.try_get::<i32, _>(i) {
                record.insert(mapped_name, value.to_string());
            } else if let Ok(value) = row.try_get::<i64, _>(i) {
                record.insert(mapped_name, value.to_string());
            } else {
                // 如果都失败，尝试作为字符串获取
                if let Ok(value) = row.try_get::<String, _>(i) {
                    record.insert(mapped_name, value);
                } else {
                    record.insert(mapped_name, "".to_string());
                }
            }
        }
        resultlist.push(record);
    }


    // 返回结果
    HttpResponse::Ok().json(ListResponse {
        code: 200,
        msg: "查询成功",
        data: Some(resultlist),
        total: total.0,
    })
}


// 通用新增公共方法
pub async fn create_api(
    pool: &sqlx::MySqlPool,
    table_name: &str,
    data: &std::collections::HashMap<String, String>,
    unique_fields: &[String],
) -> HttpResponse {
    // 1. 检查唯一性约束
    for field in unique_fields {
        if let Some(value) = data.get(field) {
            let exists: Result<(i64,), _> = sqlx::query_as(&format!("SELECT COUNT(*) FROM {} WHERE {} = ?", table_name, field))
                .bind(value)
                .fetch_one(pool)
                .await;
            match exists {
                Ok((count,)) if count > 0 => {
                    return HttpResponse::Ok().json(BasicResponse {
                        code: 400,
                        msg: "数据已存在",
                    });
                }
                Err(e) => {
                    eprintln!("唯一性检查失败: {:?}", e);
                    return HttpResponse::InternalServerError().json(BasicResponse {
                        code: 500,
                        msg: "唯一性检查失败",
                    });
                }
                _ => {}
            }
        }
    }

    // 2. 处理密码加密（如果 data 里有 "password" 字段）
    let mut processed_data = data.clone();
    if let Some(password) = processed_data.get("password") {
        let hashed_pwd = match hash(password, DEFAULT_COST) {
            Ok(pwd) => pwd,
            Err(_) => {
                return HttpResponse::InternalServerError().json(BasicResponse {
                    code: 500,
                    msg: "密码加密失败",
                });
            }
        };
        processed_data.insert("password".to_string(), hashed_pwd);
    }

    // 3. 构建插入SQL
    let fields: Vec<String> = processed_data.keys().cloned().collect();
    let placeholders: Vec<String> = fields.iter().map(|_| "?".to_string()).collect();
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        fields.join(", "),
        placeholders.join(", ")
    );

    // 4. 执行插入
    let mut query = sqlx::query(&sql);
    for field in &fields {
        if let Some(value) = processed_data.get(field) {
            query = query.bind(value);
        }
    }

    match query.execute(pool).await {
        Ok(_) => HttpResponse::Ok().json(BasicResponse {
            code: 200,
            msg: "新增成功",
        }),
        Err(e) => {
            eprintln!("新增失败: {:?}", e);
            HttpResponse::InternalServerError().json(BasicResponse {
                code: 500,
                msg: "新增失败",
            })
        }
    }
}


// 通用详情
pub async fn detail_api<T>(
    pool: web::Data<MySqlPool>,
    table: &str,
    pk_field: &str,
    id: i32,
) -> HttpResponse
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + serde::Serialize + Unpin + Send,
{
    let query = format!("SELECT * FROM {} WHERE {} = ?", table, pk_field);
    let result = sqlx::query_as::<_, T>(&query)
        .bind(id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(ApiDetailResponse {
            code: 200,
            msg: "查询成功",
            data: data,
        }),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(BasicResponse {
            code: 404,
            msg: "数据不存在"
        }),
        Err(e) => {
            eprintln!("查询详情失败: {:?}", e);
            HttpResponse::InternalServerError().json(BasicResponse {
                code: 500,
                msg: "查询失败"
            })
        }
    }
}



// 通用更新接口
pub async fn update_api(
    pool: &MySqlPool,
    table: &str,
    pk_field: &str,
    pk_value: i32,
    data: &HashMap<String, String>,
) -> HttpResponse {
    // 构建 SET 语句
    let sets: Vec<String> = data.keys().map(|k| format!("{} = ?", k)).collect();
    let sql = format!(
        "UPDATE {} SET {} WHERE {} = ?",
        table,
        sets.join(", "),
        pk_field
    );

    let mut query = sqlx::query(&sql);
    for key in data.keys() {
        query = query.bind(data.get(key).unwrap());
    }
    query = query.bind(pk_value);

    match query.execute(pool).await {
        Ok(_) => HttpResponse::Ok().json(BasicResponse {
            code: 200,
            msg: "更新成功!",
        }),
        Err(e) => {
            eprintln!("数据库更新失败: {:?}", e);
            HttpResponse::InternalServerError().json(BasicResponse {
                code: 500,
                msg: "更新失败!",
            })
        }
    }
}



// 通用删除--软删除和真删除
pub async fn delete_api(
    pool: &MySqlPool,
    table: &str,
    pk_field: &str,
    pk_value: i32,
    soft_delete: bool, // 新增参数，true=软删除，false=真删除
) -> HttpResponse {
    let sql;
    let mut query;

    if soft_delete {
        // 假删除，isDeleted 字段置为 1
        sql = format!("UPDATE {} SET del_flag = ? WHERE {} = ?", table, pk_field);
        query = sqlx::query(&sql)
            .bind("1".to_string()) // 1 表示已删除
            .bind(pk_value);
    } else {
        // 真删除
        sql = format!("DELETE FROM {} WHERE {} = ?", table, pk_field);
        query = sqlx::query(&sql)
            .bind(pk_value);
    }

    let result = query.execute(pool).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(BasicResponse {
            code: 200,
            msg: "删除成功!",
        }),
        Err(e) => {
            eprintln!("数据库删除失败: {:?}", e);
            HttpResponse::InternalServerError().json(BasicResponse {
                code: 500,
                msg: "删除失败!",
            })
        }
    }
}



