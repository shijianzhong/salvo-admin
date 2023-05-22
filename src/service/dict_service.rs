use crate::entity::sys_dict_data_entity::SysDictData;
use salvo::oapi::extract::JsonBody;
use crate::entity::sys_user_entity::SysUser;
use crate::mapper::dict_mapper;
use crate::GLOBAL_DB;
use crate::entity::sys_dict_type_entity::{SysDictType,ModifySysDictType};
use crate::model::common_model::Page;
use crate::utils::func;
use rbatis::rbdc::datetime::DateTime;

pub async fn get_dict_by_page(page_num:u64,page_size:u64,dict_name:&str,dict_type:&str,status:&str,begin_time:&str,end_time:&str)->rbatis::Result<Page<SysDictType>>{

  let (num,size) = func::create_page(page_num, page_size);
  let list:Vec<SysDictType> = dict_mapper::select_dcit_type_by_page(&mut GLOBAL_DB.clone(),dict_name,dict_type,status,begin_time,end_time,num,size).await?;
  let count:u64 = dict_mapper::select_dcit_type_by_count(&mut GLOBAL_DB.clone(),dict_name,dict_type,status,begin_time,end_time).await?;
  Ok(Page { rows: list, total: count })
}

pub async fn get_dict_data_by_type(dict_type:&str)->rbatis::Result<Vec<SysDictData>>{
  let list = dict_mapper::select_dict_data_by_type(&mut GLOBAL_DB.clone(),dict_type).await?;
  Ok(list)
}

pub async fn get_dict_by_id(dict_id:i64)->rbatis::Result<Option<SysDictType>>{
  let st = dict_mapper::select_dict_by_id(&mut GLOBAL_DB.clone(),dict_id).await?;
  Ok(st)
}

pub async fn add_dict_type(user_id:i32,dict_name:String,dict_type:String,status:String,remark:Option<String>)->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let sys_dict_type = ModifySysDictType{dict_id:0,dict_name,dict_type,status,create_by:user.user_name.clone(),create_time:DateTime::now(),update_by:None,update_time:None,remark:remark};
  let rows = ModifySysDictType::insert(&mut GLOBAL_DB.clone(), &sys_dict_type).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn edit_dict_type(user_id:i32,dict:JsonBody<ModifySysDictType>)->rbatis::Result<bool>{
  let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
  let user = user.get(0).unwrap();
  let sys_dict_type = ModifySysDictType{
    dict_id:dict.dict_id,
    dict_name:dict.dict_name.clone(),
    dict_type:dict.dict_type.clone(),
    status:dict.status.clone(),
    create_by:dict.create_by.clone(),
    create_time:dict.create_time.clone(),
    update_by:Some(user.user_name.clone()),
    update_time:Some(DateTime::now()),
    remark:dict.remark.clone()
  };
  let rows = ModifySysDictType::update_by_column(&mut GLOBAL_DB.clone(), &sys_dict_type,"dict_id").await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}

pub async fn del_dict_type(dict_id:Vec<&str>)->rbatis::Result<bool>{
  let rows = dict_mapper::del_dict_by_id(&mut GLOBAL_DB.clone(),dict_id).await?;
  Ok(func::is_modify_ok(rows.rows_affected))
}