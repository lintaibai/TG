use actix_web::{HttpResponse, http::StatusCode};
use serde::{Serialize, Deserialize};

// 统一响应结构体
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

/// 200 成功响应 (带数据)
pub fn success<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        msg: "操作成功".to_string(),
        data: Some(data),
    })
}

/// 200 成功响应 (仅消息)
pub fn success_msg(msg: &str) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 200,
        msg: msg.to_string(),
        data: None,
    })
}

/// 201 创建成功
pub fn created<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Created().json(ApiResponse {
        code: 201,
        msg: "资源创建成功".to_string(),
        data: Some(data),
    })
}

/// 204 无内容 (用于删除成功等场景)
pub fn no_content() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

/// 400 错误请求
pub fn bad_request(msg: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ApiResponse::<()> {
        code: 400,
        msg: msg.to_string(),
        data: None,
    })
}
// 错误请求带错误提示
// pub fn bad_request_data(msg: &str) -> HttpResponse {
//     HttpResponse::BadRequest().json(ApiResponse::<()> {
//         code: 400,
//         msg: msg.to_string(),
//         data: None,
//     })
// }


/// 401 未授权
pub fn unauthorized(msg: &str) -> HttpResponse {
    HttpResponse::Unauthorized().json(ApiResponse::<()> {
        code: 401,
        msg: msg.to_string(),
        data: None,
    })
}

/// 403 禁止访问
pub fn forbidden(msg: &str) -> HttpResponse {
    HttpResponse::Forbidden().json(ApiResponse::<()> {
        code: 403,
        msg: msg.to_string(),
        data: None,
    })
}

/// 404 资源未找到
pub fn not_found(msg: &str) -> HttpResponse {
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 404,
        msg: msg.to_string(),
        data: None,
    })
}

/// 422 请求格式正确但语义错误
pub fn unprocessable_entity(msg: &str) -> HttpResponse {
    HttpResponse::UnprocessableEntity().json(ApiResponse::<()> {
        code: 422,
        msg: msg.to_string(),
        data: None,
    })
}

/// 500 服务器内部错误
pub fn internal_server_error(msg: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiResponse::<()> {
        code: 500,
        msg: msg.to_string(),
        data: None,
    })
}

/// 503 服务不可用
pub fn service_unavailable(msg: &str) -> HttpResponse {
    HttpResponse::ServiceUnavailable().json(ApiResponse::<()> {
        code: 503,
        msg: msg.to_string(),
        data: None,
    })
}