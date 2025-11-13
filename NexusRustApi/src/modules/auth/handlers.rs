#[allow(unused_imports)]
use actix_web::{web, HttpRequest, HttpResponse, Responder};

#[allow(unused_imports)]
// use sqlx::{MySqlPool, Row}; // 添加 Row trait

#[allow(unused_imports)]
use sqlx::{MySqlPool,MySql, Pool};

#[allow(unused_imports)]
use crate::common::response::ApiResponse; // 导入 ApiResponse 模型

#[allow(unused_imports)]
use crate::common::response::BasicResponse; // 导入 BasicResponse 模型

// token部分
const JWT_SECRET: &[u8] = b"your_secret_key";
#[allow(unused_imports)]
use crate::common::response::Claims;
#[allow(unused_imports)]
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
#[allow(unused_imports)]
use crate::modules::user::models::User; // 导入 User 模型

//结构体使用
use serde::{Serialize, Deserialize}; 

#[derive(sqlx::FromRow, Serialize)] 
pub struct Route {
    pub id: i32,
    pub path: String,
    pub component: String,
    // 其他路由字段...
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Role {
    pub role_id: i32,
    // 其他角色字段...
}



#[derive(sqlx::FromRow)]
pub struct MenuId {
    pub menu_id: i32,
}


// 详情数据模型
#[derive(Debug, Serialize)]
pub struct MenuResponse<T: Serialize> {
    pub code: i32,
    pub msg: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}


 #[derive(sqlx::FromRow, Serialize, Debug, Clone)]
pub struct Menu {
    pub menu_id: i32,                    // 菜单ID
    pub menu_name: String,               // 菜单名称
    pub parent_id: i32,                // 父菜单ID
    pub order_num: i32,                 // 显示顺序
    pub path: String,                   // 路由地址
    pub component: Option<String>,      // 组件路径
    pub query: Option<String>,          // 路由参数
    pub is_frame: i32,                 // 是否为外链（0是 1否）
    pub is_cache: i32,                 // 是否缓存（0缓存 1不缓存）
    pub menu_type: String,              // 菜单类型（M目录 C菜单 F按钮）
    pub visible: String,                 // 菜单状态（0显示 1隐藏）
    // pub status: i32,                  // 菜单状态（0正常 1停用）
    pub perms: Option<String>,         // 权限标识
    pub icon: String,                  // 菜单图标
    // #[serde(skip)]
    // pub create_by: String,             // 创建者
    // #[serde(skip)]
    // pub create_time: Option<NaiveDateTime>, // 创建时间
    // #[serde(skip)]
    pub update_by: String,             // 更新者
    // #[serde(skip)]
    // pub update_time: Option<NaiveDateTime>, // 更新时间
    // #[serde(skip)]
    pub remark: String,                // 备注
}

// 返回路由结构体
#[derive(Debug, Serialize, Deserialize,Clone)]
struct MenuMeta {
    pub title: String,
    pub icon: String,
    pub no_cache: bool,
    pub link: Option<String>,
    pub show: bool,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
struct MenuItem {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub hidden: bool,
    pub component: Option<String>,
    pub meta: MenuMeta,
    pub children: Vec<MenuItem>,
}
#[derive(sqlx::FromRow, Serialize, Debug, Clone)]
pub struct MenuTree {
    pub id: i64,                        // 菜单ID
    pub name: String,                   // 菜单名称
    pub path: String,                   // 路由地址
    pub component: Option<String>,      // 组件路径
    pub parent_id: i64,                 // 父菜单ID
    pub sort: i32,                      // 显示顺序
    pub visible: String,                  // 是否可见
    pub permission: Option<String>,     // 权限标识
    pub icon: String,                  // 菜单图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuTree>>, // 子菜单
}

// 扁平化菜单转树形结构
fn build_menu_tree(menus: Vec<Menu>) -> Vec<MenuItem> {
    let mut menu_map = std::collections::HashMap::new();
    let mut root_menus = Vec::new();
    
    // 将菜单按ID分组
    for menu in &menus {
        menu_map.insert(menu.menu_id, MenuItem {
            id: menu.menu_id as i32,
            name: menu.menu_name.clone(),
            // path: menu.path.clone(),
            path: "/".to_string() + &menu.path,
            hidden: menu.menu_type != "M",
            component: menu.component.clone(),
            meta: MenuMeta {
                show: menu.visible == "0", // "0" 显示 "1" 隐藏
                title: menu.menu_name.clone(),
                icon: menu.icon.clone(),
                no_cache: menu.is_cache == 1,
                link: menu.query.as_ref().map_or(None, |q| if q.is_empty() { None } else { Some(q.clone()) }),
            },
            children: Vec::new(),
        });
    }
    
    // 构建父子关系映射
    let mut parent_child_map = std::collections::HashMap::new();
    for menu in &menus {
        parent_child_map.entry(menu.parent_id).or_insert_with(Vec::new).push(menu.menu_id);
    }
    
    // 递归构建树形结构
    fn build_tree(
        menu_id: i32,
        menu_map: &std::collections::HashMap<i32, MenuItem>,
        parent_child_map: &std::collections::HashMap<i32, Vec<i32>>,
    ) -> MenuItem {
        let mut menu = menu_map.get(&menu_id).unwrap().clone();

        if let Some(child_ids) = parent_child_map.get(&menu_id) {
            menu.children = child_ids
                .iter()
                .filter_map(|&id| {
                    if let Some(child) = build_tree_opt(id, menu_map, parent_child_map) {
                        Some(child)
                    } else {
                        None
                    }
                })
                .collect();
        }
        menu
    }

    fn build_tree_opt(
        menu_id: i32,
        menu_map: &std::collections::HashMap<i32, MenuItem>,
        parent_child_map: &std::collections::HashMap<i32, Vec<i32>>,
    ) -> Option<MenuItem> {
        Some(build_tree(menu_id, menu_map, parent_child_map))
    }
    
    // 构建根菜单
    for menu in &menus {
        if menu.parent_id == 0 {
            root_menus.push(build_tree(menu.menu_id, &menu_map, &parent_child_map));
        }
    }

    // 排序函数
    fn sort_menu_items(menus: &mut Vec<MenuItem>) {
        menus.sort_by_key(|m| m.id);
        for menu in menus {
            sort_menu_items(&mut menu.children);
        }
    }

    sort_menu_items(&mut root_menus);
    root_menus
}

pub async fn get_routers(
    req: HttpRequest,
    pool: web::Data<MySqlPool>
) -> HttpResponse {
    // 从 header 获取 token
    let token = match req.headers().get("Authorization") {
        Some(t) => t.to_str().unwrap_or("").replace("Bearer ", ""),
        None => return HttpResponse::Unauthorized().json(ApiResponse {
            code: 401,
            msg: "未提供Token",
            data: None::<()>,
        }),
    };
    print!("token: {}", token);


    // 校验 token
    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    ) {
        Ok(data) => {
            // 打印解码后的 token 信息
            println!("Token 解码成功: {:?}", data.claims);
            data
        },
        Err(_) => return HttpResponse::Unauthorized().json(ApiResponse {
            code: 401,
            msg: "Token无效或已过期",
            data: None::<()>,
        }),
    };
  
    // 根据 token 里的 username 查询用户信息
    let user = match sqlx::query_as::<_, User>(
        "SELECT * FROM sys_user WHERE username = ?"
    )
    .bind(&token_data.claims.username)
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(data) => {
            // 打印解码后的 token 信息
            println!("User解码=====: {:?}", data.user_id);
            data
        },
        Err(_) => return HttpResponse::Unauthorized().json(ApiResponse {
            code: 401,
            msg: "用户不存在",
            data: None::<()>,
        }),
    };


    // 查询用户角色
    let roles: Vec<Role> = match sqlx::query_as::<_, Role>(
        "SELECT * FROM sys_role r 
         INNER JOIN sys_user_role ur ON r.role_id = ur.role_id 
         WHERE ur.user_id = ?"
    )
    .bind(user.user_id)
    .fetch_all(pool.get_ref())
    .await
    {   
        Ok(data) => {
            // 打印解码后的 token 信息
            println!("用户角色==== {:?}", data);
            data
        },
        // Ok(roles) => roles,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse {
            code: 500,
            msg: "获取用户角色失败",
            data: None::<()>,
        }),
    };
    

    // 收集所有角色ID
    let role_ids: Vec<i32> = roles.iter().map(|role| role.role_id).collect();
  
   
    // 查询这些角色对应的菜单ID
    let menu_ids: Vec<i32> = if role_ids.is_empty() {
        Vec::new() // 如果没有角色，返回空数组
    } else {
        // 创建 IN 查询的占位符
        let placeholders: String = role_ids.iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        
        // 使用占位符构建查询
        let query = format!(
            "SELECT DISTINCT menu_id FROM sys_role_menu WHERE role_id IN ({})",
            placeholders
        );

        // 创建查询并绑定每个参数
        let mut query_builder = sqlx::query_as::<_, MenuId>(&query);
        for role_id in &role_ids {
            query_builder = query_builder.bind(role_id);
        }
        
        match query_builder.fetch_all(pool.get_ref()).await {
            Ok(menu_ids) => menu_ids.into_iter().map(|menu_id| menu_id.menu_id).collect(),
            Err(_) => return HttpResponse::InternalServerError().json(ApiResponse {
                code: 500,
                msg: "获取用户菜单失败",
                data: None::<()>,
            }),
        }
    };

    println!("用户菜单ID==== {:?}", menu_ids);

    // 对菜单ID进行去重
    let unique_menu_ids: Vec<i32> = menu_ids.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();

    println!("用户菜单去重==== {:?}", unique_menu_ids);

    // 查询菜单信息
    let menus: Vec<Menu> = if unique_menu_ids.is_empty() {
        Vec::new()
    } else {
        // 创建 IN 查询的占位符
        let placeholders: String = unique_menu_ids.iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        
        // 使用占位符构建查询
        let query = format!(
            "SELECT * FROM sys_menu WHERE menu_id IN ({})  AND `menu_type` != 'F'
            ORDER BY parent_id, order_num",
            placeholders
        );
        
        // 创建查询并绑定每个参数
        let mut query_builder = sqlx::query_as::<_, Menu>(&query);
        for menu_id in &unique_menu_ids {
            query_builder = query_builder.bind(menu_id);
        }
        
        match query_builder.fetch_all(pool.get_ref()).await {
            Ok(menus) => {
                // 打印解码后的 token 信息
                println!("用户menus==== {:?}", menus);
                menus
            },
            Err(err) => {
                println!("用户err==== {:?}", err);
                return HttpResponse::InternalServerError().json(ApiResponse {
                    code: 500,
                    msg: "转化菜单信息失败",
                    data: None::<()>,
                })
            },
        }
    };

    // 将菜单信息组合为树形结构
    let menus = build_menu_tree(menus);
    // print!("menus: {:?}", menus);
    HttpResponse::Ok().json(MenuResponse {
        code: 200,
        msg: "获取成功",
         data: Some(menus),
    })

}
     
