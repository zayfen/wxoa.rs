use crate::models::{UserDetailsInfo, UserInfo};
use crate::DbConn;
use regex::Regex;
use sms_service::{check_auth_code, send_sms_async};
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum UserState {
  BeforeRegister = 0,
  RegisteringWaitForMobile = 1,
  RegisteringWaitForAuthCode = 2,
  CompleteRegister = 3,
}

/// # 处理命令
pub fn dispatch_command(conn: &DbConn, map: &HashMap<String, String>, command: &String) -> String {
  let open_id = &map["FromUserName"];
  // step0: 判断注册状态
  let state = UserInfo::get_user_state(conn, &open_id);

  // 提示去注册
  if state == UserState::BeforeRegister as i32 {
    // step1： command == “注册”，开启注册流程，录入{openid}到注册流程
    if command == "register" || command == "注册" {
      return handle_before_register(conn, &open_id);
    }
    return "您还没有注册，请回复[注册]两个字进行注册流程！".to_owned();
  }

  // 在注册流程中, 等待输入手机号，发送验证码
  if state == UserState::RegisteringWaitForMobile as i32 {
    let mobile = command;
    return handle_registering_waiting_mobile(conn, &open_id, &mobile);
  }

  // 在注册流程中，等待验证码， 验证码错误，回到等待输入姓名手机号流程，更改state
  if state == UserState::RegisteringWaitForAuthCode as i32 {
    let auth_code = command;
    return handle_registering_waiting_authcode(conn, &open_id, &auth_code);
  }

  // 已经注册完成
  if state == UserState::CompleteRegister as i32 {
    if command == "年假" || command == "调休" {
      return handle_query_days(conn, &open_id);
    }

    return "不识别的指令(更多指令开发中... ^_^)".to_owned();
  }

  println!("command {:?}", &command);
  "Oooooh! 您处于不合法的状态中".to_owned()
}

fn update_user_state(conn: &DbConn, open_id: &String, state: UserState) -> bool {
  let user_result: diesel::QueryResult<UserInfo> = UserInfo::get_user_info_by_openid(conn, open_id);
  if let Ok(mut user) = user_result {
    user.f_state = state as i32;
    let rows_count = UserInfo::update_user_info(conn, user.clone());
    println!(
      "change_user_state: {:?}; affected_rows: {:?}",
      &user, &rows_count
    );
    return rows_count.is_ok();
  } else {
    return false;
  }
}

fn update_user_auth_code(conn: &DbConn, open_id: &String, auth_code: &String) -> bool {
  let user_result: diesel::QueryResult<UserInfo> = UserInfo::get_user_info_by_openid(conn, open_id);
  if let Ok(mut user) = user_result {
    user.f_remark = Some(auth_code.to_string());
    let rows_count = UserInfo::update_user_info(conn, user.clone());
    println!(
      "change_user_auth_code: {:?}; affected_rows: {:?}",
      &user, &rows_count
    );
    return rows_count.is_ok();
  } else {
    return false;
  }
}

fn update_user_mobile(conn: &DbConn, open_id: &String, mobile: &String) -> bool {
  let user_result: diesel::QueryResult<UserInfo> = UserInfo::get_user_info_by_openid(conn, open_id);
  if let Ok(mut user) = user_result {
    user.f_mobile = mobile.to_owned();
    let rows_count = UserInfo::update_user_info(conn, user.clone());
    println!(
      "change_user_mobile: {:?}; affected_rows: {:?}",
      &user, &rows_count
    );
    return rows_count.is_ok();
  } else {
    return false;
  }
}

/// 开始注册流程
fn handle_before_register(conn: &DbConn, open_id: &String) -> String {
  // 插入一条新的记录，并将state设置为1
  let mut new_user = UserInfo::default();
  new_user.f_open_id = open_id.clone();
  new_user.f_state = UserState::RegisteringWaitForMobile as i32;
  let affected_rows = UserInfo::insert_user_info(conn, &new_user);
  if affected_rows.is_ok() {
    "开启注册流程, 请输入在公司登记的真实手机号，如:\n13286661539".to_owned()
  } else {
    "服务器内部错误，请稍后重试".to_owned()
  }
}

// 等待输入手机号
fn handle_registering_waiting_mobile(conn: &DbConn, open_id: &String, mobile: &String) -> String {
  lazy_static! {
    static ref MobileRE: Regex = Regex::new(r"^1\d{10}$").unwrap();
  }
  let valid_mobile = MobileRE.is_match(mobile);
  if valid_mobile {
    // 发送验证码
    send_sms_async(mobile.clone());
    // 存储手机号
    let mobile_update_result = update_user_mobile(conn, open_id, &mobile);

    // 更新状态到 等待验证码
    let state_update_result =
      update_user_state(conn, open_id, UserState::RegisteringWaitForAuthCode);

    if state_update_result && mobile_update_result {
      return format!("您的验证码已发送，请回复验证码:");
    } else {
      update_user_state(conn, open_id, UserState::RegisteringWaitForMobile);
      return format!("验证码发送失败，请重新回复手机号");
    }
  }
  "请输入正确的手机号".to_owned()
}

// 等待输入验证码
fn handle_registering_waiting_authcode(
  conn: &DbConn,
  open_id: &String,
  auth_code: &String,
) -> String {
  let user_result = UserInfo::get_user_info_by_openid(conn, open_id);
  if let Ok(user) = user_result {
    let mobile = &user.f_mobile;
    if check_auth_code(mobile, auth_code).is_ok() {
      // 验证成功
      update_user_state(conn, open_id, UserState::CompleteRegister);
      return String::from("注册成功！");
    } else {
      // 验证失败
      update_user_state(conn, open_id, UserState::RegisteringWaitForMobile);
      return String::from("验证码错误,回退到输入手机号状态，请重新回复手机号");
    }
  } else {
    // 没有找到这个人，重新回复到第一步
    return "您的信息被删除了，你需要重新注册, 输入[注册]两个字开始注册".to_owned();
  }
}

// 查询假期
fn handle_query_days(conn: &DbConn, open_id: &String) -> String {
  let data = UserInfo::get_user_info_by_openid(conn, open_id);
  if data.is_ok() {
    let _data = data.unwrap();
    let mobile = _data.f_mobile;

    let details_result = UserDetailsInfo::get_lastest_user_details_by_mobile(conn, &mobile);
    match details_result {
      Ok(details) => {
        let name = details.f_name;
        let annual_leave_days = details.f_annual_leave_days;
        let rest_days = details.f_rest_days;
        format!(
          "{}您好，您的年假剩余{}天，调休剩余{}天",
          name.unwrap_or(mobile),
          annual_leave_days,
          rest_days
        )
      }
      _ => "未查询到您的假期,请联系管理员录入".to_owned(),
    }
  } else {
    "未查询到您的假期,请联系管理员录入".to_owned()
  }
}
