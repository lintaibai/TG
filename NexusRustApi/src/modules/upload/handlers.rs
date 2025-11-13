use actix_web::{HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::StreamExt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use uuid::Uuid;
use std::env;
use serde::{Serialize};


// 导入上传模型
// use crate::common::response::UploadResponse;

// 定义响应数据结构
#[derive(serde::Serialize)]
struct UploadResponse {

    #[serde(rename = "fullPath")]
    full_path: String,

    #[serde(rename = "relativePath")]
    relative_path: String,
    size: u64,

    #[serde(rename = "fileName")]
    file_name: String,

    #[serde(rename = "fileType")]
    file_type: String,

    #[serde(rename = "fileUid")]
    file_uid:String,
}


// 统一响应结构体
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

const BASE_UPLOAD_DIR: &str = "./uploads";
const IMAGE_SUBDIR: &str = "images";  // 图片存储子目录
const ALLOWED_MIME_TYPES: [&str; 3] = ["image/jpeg", "image/png", "image/gif"];

pub async fn upload_img(mut payload: Multipart) -> impl Responder {
    // 创建图片存储目录（如果不存在）
    let image_dir = format!("{}/{}", BASE_UPLOAD_DIR, IMAGE_SUBDIR);
    if let Err(e) = create_dir_all(&image_dir) {
        return internal_server_error(&format!("创建目录失败: {}", e));
    }

    // 获取基础URL（从环境变量或使用默认值）
    let base_url = env::var("BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    // 遍历多部分表单字段
    while let Some(field_result) = payload.next().await {
        let mut field = match field_result {
            Ok(f) => f,
            Err(e) => return bad_request(&format!("字段解析失败: {}", e)),
        };

        // 获取内容处置头部
        let content_disposition = field.content_disposition();
        
        // 获取文件名
        let original_file_name = match content_disposition.get_filename() {
            Some(name) => name.to_string(),
            None => continue,  // 跳过非文件字段
        };

        // 验证文件类型
        let mime_type = field.content_type().to_string();
        if !ALLOWED_MIME_TYPES.contains(&mime_type.as_str()) {
            return bad_request("只允许 JPEG、PNG 或 GIF 图片");
        }

        // 生成唯一文件名和路径
        let extension = get_extension(&mime_type);
        let file_id = Uuid::new_v4().to_string();
        let unique_name = format!("{}.{}", file_id, extension);
        
        // 文件存储路径（包含子目录）
        let file_path = format!("{}/{}", image_dir, unique_name);
        
        // URL 路径（包含子目录）
        let relative_path = format!("/uploads/{}/{}", IMAGE_SUBDIR, unique_name);
        let absolute_path = format!("{}{}", base_url, relative_path);

        // 保存文件内容并获取文件大小
        let file_size = match save_file(&mut field, &file_path).await {
            Ok(size) => size,
            Err(e) => {
               return internal_server_error(&format!("文件保存失败: {}", e));
            }
        };

        // 创建响应数据
        let response_data: UploadResponse = UploadResponse {
            full_path: absolute_path,
            relative_path: relative_path,
            size: file_size,
            file_name: format!("图片-{}", original_file_name),
            file_type: mime_type,
            file_uid: file_id,
        };

        // 返回成功响应
        return HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "图片上传成功".to_string(),
            data: Some(response_data),
        });
    }

    // 没有找到有效的文件字段
    bad_request("未检测到上传的文件")
}

/// 根据 MIME 类型获取文件扩展名
fn get_extension(mime_type: &str) -> &str {
    match mime_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        _ => "bin", // 不会发生（前面已验证）
    }
}

/// 保存上传的文件并返回文件大小
async fn save_file(field: &mut actix_multipart::Field, path: &str) -> std::io::Result<u64> {
    let mut file = File::create(path)?;
    let mut total_size = 0;
    
    // 处理每个数据块
    while let Some(chunk_result) = field.next().await {
        let chunk = chunk_result.map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other, 
                format!("读取数据块失败: {}", e)
            )
        })?;
        
        // 写入文件并更新大小
        file.write_all(&chunk)?;
        total_size += chunk.len() as u64;
    }
    
    file.flush()?;
    Ok(total_size)
}

/// 400 错误响应
fn bad_request(msg: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ApiResponse::<()> {
        code: 400,
        msg: msg.to_string(),  // 使用实际错误消息
        data: None,
    })
}

/// 500 错误响应

/// 500 错误响应
fn internal_server_error(msg: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiResponse::<()> {
        code: 500,
        msg: msg.to_string(),  // 使用实际错误消息
        data: None,
    })
}