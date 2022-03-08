use crate::{CONTEXT, RB};
use crate::entity::sys_entitys::CommonField;
use crate::{
    dto::sys_params_dto::SysParamsDTO, entity::sys_entitys::SysParams, request::SysParamsQuery,
};

use super::{crud_service::CrudService};

/**
 *struct:SysParamsService
 *desc:系统参数基础服务
 *author:String
 *email:348040933@qq.com
 */

pub struct SysParamsService {}
impl  Default for SysParamsService {
    fn default() -> Self {
        SysParamsService{}
    }
}
impl CrudService<SysParams, SysParamsDTO, SysParamsQuery> for SysParamsService {
    fn get_wrapper(arg: &SysParamsQuery) -> rbatis::wrapper::Wrapper {
        RB.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysParams) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
