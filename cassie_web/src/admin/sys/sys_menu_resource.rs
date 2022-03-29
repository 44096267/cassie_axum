use crate::service::crud_service::CrudService;
use crate::service::ServiceContext;
use crate::CONTAINER;
use axum::routing::get;
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json, Router,
};
use cassie_common::RespVO;
use cassie_domain::dto::sys_menu_dto::SysMenuDTO;
use cassie_domain::entity::PageData;
use cassie_domain::request::SysMenuQuery;

/**
 *method:/menu
 *desc:菜单服务 分页查询
 *author:String
 *email:348040933@qq.com
 */

pub async fn page(arg: Option<Query<SysMenuQuery>>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    let arg = arg.unwrap();
    let vo = context
        .sys_menu_service
        .page(
            &arg,
            PageData {
                page_no: arg.page.clone(),
                page_size: arg.limit.clone(),
            },
        )
        .await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn list() -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    let vo = context.sys_menu_service.menu_list().await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn nav() -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    let vo = context.sys_menu_service.get_user_menu_list().await;
    RespVO::from_result(&vo).resp_json()
}

/**
 *method:get_by_id
 *desc:菜单服务id获取
 *author:String
 *email:348040933@qq.com
 */
pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    let dto = context.sys_menu_service.get(id).await;
    RespVO::from_result(&dto).resp_json()
}
pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    context.sys_menu_service.del(&id).await;
    RespVO::from(&"删除成功".to_string()).resp_json()
}

/**
 *method:save
 *desc:菜单服务保存
 *author:String
 *email:348040933@qq.com
 */

pub async fn save(Json(arg): Json<SysMenuDTO>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    context.sys_menu_service.save_or_update(arg).await;
    RespVO::from(&"更新成功".to_string()).resp_json()
}

pub fn init_router() -> Router {
    Router::new()
        .route("/menu/list", get(list))
        .route("/menu/nav", get(nav))
        .route("/menu/:id", get(get_by_id).delete(delete))
        .route("/menu", get(page).post(save).put(save))
}
