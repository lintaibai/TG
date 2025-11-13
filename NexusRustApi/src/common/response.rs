use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use validator::{Validate, ValidationError};
// Serialize = 序列化 = “写出去”
// Deserialize = 反序列化 = “读进来”



// BasicResponse 基础返回
#[derive(Serialize)]
pub struct BasicResponse {
    pub code: i32,
    pub msg: &'static str,
}


// DataResponse 数据返回 serde反序列化
// #[derive(serde::Deserialize)]

// 详情数据模型
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub msg: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}


// 列表数据模型
#[derive(Serialize)]
pub struct ListResponse<T> {
    pub code: i32,
    pub msg: &'static str,
    pub data: Option<Vec<T>>,
    pub total: i64,
}

// 分页数据模型 
#[derive(Deserialize,Debug)]
pub struct Pagination {
    #[serde(rename = "pageNum")]
    pub page_num: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
}



// token部分
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub exp: usize,
}

// 登录请求体
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// 登录返给前端
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub code: i32,
    pub msg: &'static str,
}

// 注册请求体
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

// 注册返回
#[derive(Serialize)]
pub struct RegisterResponse {
    pub token: String,
    pub code: i32,
    pub msg: &'static str,
}

// 详情数据接口
#[derive(Serialize)]
pub struct ApiDetailResponse<T> {
    pub code: i32,
    pub msg: &'static str,
    pub data: T,
}


#[derive(Debug,Deserialize,Serialize,Clone)]
#[allow(dead_code)]
pub struct ListQuery {
    // 动态字段存储查询条件
    pub filters: Option<HashMap<String, String>>,
}

// 查询条件参数
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ListQueryParams {
    pub username: Option<String>,  // 用户名字段
    pub user_age: Option<i32>,     // 年龄字段
}

// 查询条件参数
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct QueryParams {
    pub name: Option<String>,
    pub age: Option<String>,
    pub sex: Option<String>,
    #[serde(rename = "pageNum")]
    pub page_num: Option<String>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<String>,
}


// 图片上传成功返回
#[derive(Serialize)]
pub struct UploadData {
    pub code: i32,
    
}
#[derive(Serialize)]
pub struct UploadResponse {
    pub code: i32,
    pub msg: &'static str,
    pub data: UploadData,
}



// 用户部分
// 添加用户接口返回值
#[derive(Deserialize)]
pub struct AddUserRequest {
    pub username: String, // 用户名（必填）
    pub password: String, // 密码（必填）
    pub age: Option<String>,      // 可选
    pub address: Option<String>,  // 可选
    pub sex: Option<i32>,        // 可选
    pub phone: Option<String>,    // 可选
    pub disease: Option<String>,  // 可选
    pub name: Option<String>,     // 可选
}
// 更新用户
#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "userId")] 
    pub user_id: i32,  // 添加 user_id 字段
    pub age: Option<String>,      // 可选
    pub sex: Option<i32>,    // 可选
    pub phone: Option<String>,  // 可选
    pub avatar: Option<String>, // 可选
    pub name: Option<String>,   // 可选
    pub address: Option<String>, // 可选
    pub disease: Option<String>, // 可选
}



// 角色请求结构体 - 根据您提供的JSON字段定义
#[derive(Debug, Deserialize)]
pub struct AddRoleRequest {
    #[serde(rename = "roleName")]
    pub role_name: String,           // 角色名称

    #[serde(rename = "roleKey")]
    pub role_key: String,            // 角色权限字符串

    #[serde(rename = "roleSort")]
    pub role_sort: i32,              // 显示顺序
    pub status: String,             // 角色状态（0正常 1停用）

    #[serde(rename = "menuIds")]
    pub menu_ids: Vec<i32>,          // 菜单ID数组

    #[serde(rename = "deptIds")] 
    pub dept_ids: Vec<i32>,          // 部门ID数组
    

    pub menu_check_strictly: bool,    // 菜单树选择项是否关联显示
    pub dept_check_strictly: bool,    // 部门树选择项是否关联显示
    pub remark: String,             // 备注
}


// 角色返回结构体 - 根据您提供的JSON字段定义
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// 用户信息
pub struct Role {
    #[serde(rename = "roleId")] 
    pub role_id: i32,  // 添加 user_id 字段

    #[serde(rename = "roleName")]
    pub role_name: String,

    #[serde(rename = "roleKey")]
    pub role_key: String,

    #[serde(rename = "roleSort")]
    pub role_sort: i32,
    pub data_scope: String,
    pub menu_check_strictly: i8,
    pub dept_check_strictly: i8,
    pub status: String,
    pub del_flag: String,
    pub create_by: String,
    pub remark: String,
}
// 更新角色
#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateRoleRequest {
    #[serde(rename = "roleId")] 
    pub role_id: i32,  // 添加 user_id 字段

    #[serde(rename = "roleName")]
    pub role_name: String,

    // 角色状态（0正常 1停用）
    pub status: String,  
    pub remark: String,
}




// 菜单模块
// 菜单基础结构体
// 角色返回结构体 - 根据您提供的JSON字段定义
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// 用户信息
pub struct Menu {
    #[serde(rename = "menuId")] 
    pub menu_id: i32,

    #[serde(rename = "parentId")]
    pub parent_id: i32,

    #[serde(rename = "menuName")]
    pub menu_name: String,

    pub icon: Option<String>, // 使用Option，因为可能是可选字段

    #[serde(rename = "menuType")]
    pub menu_type: String,

    #[serde(rename = "orderNum")]
    pub order_num: i32,

    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "component")]
    pub component: Option<String>, // 使用Option，因为可能是可选字段

    #[serde(rename = "isFrame")]
    pub is_frame: i32, //是否为外链（0是 1否）

    #[serde(rename = "isCache")]
    pub is_cache: i32, //是否缓存（0缓存 1不缓存）

    #[serde(rename = "visible")]
    pub visible: String,

    #[serde(rename = "status")]
    pub status: String,

    pub create_by: Option<String>, // 使用Option，因为可能是可选字段

    // #[serde(rename = "createTime")]
    // pub create_time: Option<chrono::DateTime<chrono::Utc>>, // 使用chrono处理时间

    pub update_by: Option<String>, // 使用Option，因为可能是可选字段

    // #[serde(rename = "updateTime")]
    // pub update_time: Option<chrono::DateTime<chrono::Utc>>,

    pub remark: Option<String>, // 使用Option，因为可能是可选字段

    // pub del_flag: Option<String>, // 使用Option，因为可能是可选字段
}

// 新增请求结构体
#[derive(Debug, Deserialize)]
pub struct AddMenuRequest {
    #[serde(rename = "parentId")]
    pub parent_id: i32,             // 父菜单ID

    #[serde(rename = "menuName")]
    pub menu_name: String,          // 菜单名称

    #[serde(rename = "icon")]
    pub icon: Option<String>,       // 菜单图标

    #[serde(rename = "menuType")]
    pub menu_type: String,          // 菜单类型（M目录 C菜单 F按钮）

    #[serde(rename = "orderNum")]
    pub order_num: i32,             // 显示顺序

    #[serde(rename = "path")]
    pub path: String,               // 路由地址

    #[serde(rename = "component")]
    pub component: Option<String>,  // 组件路径

    #[serde(rename = "isFrame")]
    pub is_frame: String,           // 是否为外链（0否 1是）

    #[serde(rename = "isCache")]
    pub is_cache: String,           // 是否缓存（0缓存 1不缓存）

    #[serde(rename = "visible")]
    pub visible: String,            // 菜单状态（0显示 1隐藏）

    #[serde(rename = "status")]
    pub status: String,             // 菜单状态（0正常 1停用）

    #[serde(rename = "perms")]
    pub perms: Option<String>,      // 权限标识

    #[serde(rename = "remark")]
    pub remark: Option<String>,     // 备注
}


// 部门结构体
#[derive(Debug, Deserialize)]
pub struct AddDept {

    #[serde(rename = "parentId")]
    pub parent_id:String,// 父部门ID

    #[serde(rename = "deptName")]
    pub dept_name: String,      // 部门名称

    #[serde(rename = "orderNum")]
    pub order_num: i32,         // 显示顺序

    #[serde(rename = "leader")]
    pub leader: String,         // 负责人

    #[serde(rename = "phone")]
    pub phone: String,          // 联系电话

    #[serde(rename = "email")]
    pub email: String,          // 邮箱

    #[serde(rename = "status")]
    pub status: String,         // 部门状态（0正常 1停用）
}
// 部门结构体
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Dept {
    #[serde(rename = "deptId")]
    pub dept_id: i64, // ID

    #[serde(rename = "parentId")]
    pub parent_id: i64,       // 父部门ID

    #[serde(rename = "deptName")]
    pub dept_name: String,      // 部门名称

    #[serde(rename = "orderNum")]
    pub order_num: i32,         // 显示顺序

    #[serde(rename = "leader")]
    pub leader: String,         // 负责人

    #[serde(rename = "phone")]
    pub phone: String,          // 联系电话

    #[serde(rename = "email")]
    pub email: String,          // 邮箱

    #[serde(rename = "status")]
    pub status: String,         // 部门状态（0正常 1停用）
}


// 更新菜单
#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateMenuRequest {
    #[serde(rename = "menuId")] 
    pub menu_id: i32,
    
    #[serde(rename = "parentId")]
    pub parent_id: i32,             // 父菜单ID

    #[serde(rename = "menuName")]
    pub menu_name: String,          // 菜单名称

    #[serde(rename = "icon")]
    pub icon: Option<String>,       // 菜单图标

    #[serde(rename = "menuType")]
    pub menu_type: String,          // 菜单类型（M目录 C菜单 F按钮）

    #[serde(rename = "orderNum")]
    pub order_num: i32,             // 显示顺序

    #[serde(rename = "path")]
    pub path: String,               // 路由地址

    #[serde(rename = "component")]
    pub component: Option<String>,  // 组件路径

    #[serde(rename = "isFrame")]
    pub is_frame: i32,          // 是否为外链（0否 1是）

    #[serde(rename = "isCache")]
    pub is_cache: i32,            // 是否缓存（0缓存 1不缓存）

    #[serde(rename = "visible")]
    pub visible: String,            // 菜单状态（0显示 1隐藏）

    #[serde(rename = "status")]
    pub status: String,             // 菜单状态（0正常 1停用）

    #[serde(rename = "perms")]
    pub perms: Option<String>,      // 权限标识

    #[serde(rename = "remark")]
    pub remark: Option<String>,     // 备注
}





// 字典类型
// 字典类型结构体
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DictType {
    #[serde(rename = "dictId")]
    pub dict_id: Option<i64>,
    
     #[serde(rename = "dictName")]
    pub dict_name: Option<String>,

    #[serde(rename = "dictType")]
    pub dict_type: Option<String>,

    #[serde(rename = "status")]
    pub status: Option<String>,

    #[serde(rename = "remark")]
    pub remark: Option<String>,
    // #[serde(rename = "dictName")]
    // pub dict_name: String,

    // #[serde(rename = "dictType")]
    // pub dict_type: String,

    // #[serde(rename = "status")]
    // pub status: String,

    // #[serde(rename = "remark")]
    // pub remark: Option<String>,

    // #[serde(rename = "createBy")]
    // pub create_by: Option<String>,

    // #[serde(rename = "createTime")]
    // pub create_time: Option<chrono::DateTime<chrono::Utc>>,

    // #[serde(rename = "updateBy")]
    // pub update_by: Option<String>,

    // #[serde(rename = "updateTime")]
    // pub update_time: Option<chrono::DateTime<chrono::Utc>>,
}

// 新增字典类型请求结构体
#[derive(Debug, Deserialize)]
pub struct AddDictTypeRequest {
    #[serde(rename = "dictName")]
    pub dict_name: Option<String>,

    #[serde(rename = "dictType")]
    pub dict_type: Option<String>,

    #[serde(rename = "status")]
    pub status: Option<String>,

    #[serde(rename = "remark")]
    pub remark: Option<String>,
}

// 更新字典类型请求结构体
#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateDictTypeRequest {

    #[serde(rename = "dictId")]
    pub dict_id: i32,

   #[serde(rename = "dictName")]
    pub dict_name: Option<String>,

    #[serde(rename = "dictType")]
    pub dict_type: Option<String>,

    #[serde(rename = "status")]
    pub status: Option<String>,

    #[serde(rename = "remark")]
    pub remark: Option<String>,
}








// 字典数据模块
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DictData {
    #[serde(rename = "dictDataId")]
    pub dict_data_id: i64,
    
    // #[serde(rename = "dictSort")]
    // pub dict_sort: i32,

    #[serde(rename = "dictLabel")]
    pub dict_label: String,

    #[serde(rename = "dictValue")]
    pub dict_value: String,

    #[serde(rename = "dictType")]
    pub dict_type: String,

    #[serde(rename = "cssClass")]
    pub css_class: Option<String>,

    #[serde(rename = "listClass")]
    pub list_class: Option<String>,

    #[serde(rename = "isDefault")]
    pub is_default: Option<String>,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "createBy")]
    pub create_by: Option<String>,

    #[serde(rename = "remark")]
    pub remark: Option<String>,

    // #[serde(rename = "createTime")]
    // pub create_time: Option<chrono::DateTime<chrono::Utc>>,

    // #[serde(rename = "updateBy")]
    // pub update_by: Option<String>,

    // #[serde(rename = "updateTime")]
    // pub update_time: Option<chrono::DateTime<chrono::Utc>>,
}

// 新增字典数据请求结构体

#[derive(Debug, Deserialize)]
pub struct AddDictDataRequest {

     #[serde(rename = "dictId")]
    pub dict_id: Option<i64>,  // bigint(0) NOT NULL AUTO_INCREMENT
    
    #[serde(rename = "dictName")]
    pub dict_name: String,     // varchar(100) DEFAULT ''
    
    #[serde(rename = "dictType")]
    pub dict_type: String,     // varchar(100) DEFAULT ''
    
    #[serde(rename = "status")]
    pub status: String,        // char(1) DEFAULT '0'
    
    #[serde(rename = "createBy")]
    pub create_by: String,     // varchar(64) DEFAULT ''
    
    // #[serde(rename = "createTime")]
    // pub create_time: Option<DateTime<Utc>>,  // datetime
    
    #[serde(rename = "updateBy")]
    pub update_by: String,     // varchar(64) DEFAULT ''
    
    // #[serde(rename = "updateTime")]
    // pub update_time: Option<DateTime<Utc>>,  // datetime
    
    #[serde(rename = "remark")]
    pub remark: String,        // varchar(500)
}

// 更新字典数据请求结构体
#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateDictDataRequest {

    #[serde(rename = "dictDataId")]
    pub dict_data_id: i32,

     #[serde(rename = "dictType")]
    pub dict_type: Option<String>,

    #[serde(rename = "dictLabel")]
    pub dict_label: Option<String>,

    #[serde(rename = "dictValue")]
    pub dict_value: Option<String>,

    #[serde(rename = "dictSort")]
    pub dict_sort: Option<i32>,

    #[serde(rename = "remark")]
    pub remark: Option<String>,

    #[serde(rename = "status")]
    pub status: Option<String>,

    // #[serde(rename = "dictCode")]
    // pub dict_code: i64,

    // #[serde(rename = "dictSort")]
    // pub dict_sort: i32,

    // #[serde(rename = "dictLabel")]
    // pub dict_label: String,

    // #[serde(rename = "dictValue")]
    // pub dict_value: String,

    // #[serde(rename = "dictType")]
    // pub dict_type: String,

    // #[serde(rename = "cssClass")]
    // pub css_class: Option<String>,

    // #[serde(rename = "listClass")]
    // pub list_class: Option<String>,

    // #[serde(rename = "isDefault")]
    // pub is_default: String,

    // #[serde(rename = "status")]
    // pub status: String,

    // #[serde(rename = "remark")]
    // pub remark: Option<String>,
}





// 公告通知模块结构体 公告通知详情
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notice {
    #[serde(rename = "noticeId")]
    pub notice_id: Option<i32>,// 通知ID

    #[serde(rename = "noticeTitle")]
    pub notice_title: String,     // 通知标题

    #[serde(rename = "noticeType")]
    pub notice_type: String,      // 通知类型

    #[serde(rename = "noticeContent")]
    pub notice_content: String,    // 通知内容

    #[serde(rename = "status")]
    pub status: String,           // 状态
}

// 公告通知模块结构体
#[derive(Debug, Deserialize)]
pub struct AddNotice {
    #[serde(flatten)]
    pub base: Notice, //继承基础的结构体信息
    // #[serde(rename = "delFlag")]
    // pub del_flag: char,   // 删除标志 ('0'-未删除, '1'-已删除)
}




//职位模块
// 返回结构体 - 根据您提供的JSON字段定义
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// 用户信息
pub struct Job {
    #[serde(rename = "jobId")] 
    pub job_id: i32,  // 添加 user_id 字段

     #[serde(rename = "jobTitle")]
    pub job_title: String,

    #[serde(rename = "department")]
    pub department: String,

    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "jobType")]
    pub job_type: String,

    #[serde(rename = "salaryRange")]
    pub salary_range: String,

    #[serde(rename = "jobStatus")]
    pub job_status: String,

    #[serde(rename = "jobRequirements")]
    pub job_requirements: Option<String>,

    #[serde(rename = "jobResponsibilities")]
    pub job_responsibilities: Option<String>,
}

// 添加职位
#[derive(Serialize, Debug, Deserialize)]
pub struct AddJobRequest {
    #[serde(rename = "jobTitle")]
    pub job_title: String,

    #[serde(rename = "department")]
    pub department: String,

    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "jobType")]
    pub job_type: String,

    #[serde(rename = "salaryRange")]
    pub salary_range: String,

    #[serde(rename = "jobStatus")]
    pub job_status: String,

    #[serde(rename = "jobRequirements")]
    pub job_requirements: Option<String>,

    #[serde(rename = "jobResponsibilities")]
    pub job_responsibilities: Option<String>,
}

// 职位更新模块
#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateJobRequest {
    #[serde(rename = "jobId")] 
    pub job_id: i32,  // 添加 user_id 字段
    
    #[serde(rename = "jobTitle")]
    pub job_title: String,

    #[serde(rename = "department")]
    pub department: String,

    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "jobType")]
    pub job_type: String,

    #[serde(rename = "salaryRange")]
    pub salary_range: String,

    #[serde(rename = "jobStatus")]
    pub job_status: String,

    #[serde(rename = "jobRequirements")]
    pub job_requirements: Option<String>,

    #[serde(rename = "jobResponsibilities")]
    pub job_responsibilities: Option<String>,
}

