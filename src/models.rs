use crate::schema::t_user_details_info;
use crate::schema::t_user_info;
use crate::DbConn;
use chrono::{prelude::*, Utc};
use diesel::prelude::*;

#[derive(serde::Serialize, Queryable, Insertable, Debug, Clone)]
#[table_name = "t_user_info"]
pub struct UserInfo {
  pub f_open_id: String,
  pub f_mobile: String,
  pub f_state: i32,
  pub f_remark: Option<String>,
}

impl UserInfo {
  pub fn default() -> UserInfo {
    UserInfo {
      f_open_id: "".to_owned(),
      f_mobile: "110".to_owned(),
      f_state: 0,
      f_remark: Some("无".to_owned()),
    }
  }

  pub fn all_user_info(conn: &DbConn) -> QueryResult<Vec<UserInfo>> {
    use crate::schema::t_user_info::dsl::t_user_info as users;
    let data: Vec<UserInfo> = users
      .load::<UserInfo>(&conn.0)
      .expect("all_user_info get error");
    Ok(data)
  }

  pub fn get_user_info_by_openid(conn: &DbConn, open_id: &String) -> QueryResult<UserInfo> {
    use crate::schema::t_user_info::dsl::{f_open_id, t_user_info as users};
    let data = users
      .filter(f_open_id.eq(open_id))
      .first::<UserInfo>(&conn.0);
    data
  }

  /// 获取用户的状态
  pub fn get_user_state(conn: &DbConn, open_id: &String) -> i32 {
    let res_user = UserInfo::get_user_info_by_openid(conn, open_id);
    match res_user {
      Ok(user) => {
        return user.f_state;
      }
      _ => 0,
    }
  }

  /// insert user
  /// 返回受影响的行数
  pub fn insert_user_info(conn: &DbConn, user_info: &UserInfo) -> QueryResult<usize> {
    use crate::schema::t_user_info::dsl::t_user_info as users;
    let rows_count = diesel::insert_into(users)
      .values(user_info)
      .execute(&conn.0);
    rows_count
  }

  /// 更新用户信息
  pub fn update_user_info(conn: &DbConn, user_info: UserInfo) -> QueryResult<usize> {
    use crate::schema::t_user_info::dsl::*;
    let _open_id = user_info.f_open_id.clone();
    let rows_count = diesel::update(t_user_info.filter(f_open_id.eq(_open_id)))
      .set((
        f_mobile.eq(user_info.f_mobile),
        f_remark.eq(user_info.f_remark),
        f_state.eq(user_info.f_state),
      ))
      .execute(&conn.0);
    rows_count
  }
}

/// 用户详细信息表
#[derive(serde::Serialize, Queryable, Insertable, AsChangeset, Debug, Clone)]
#[table_name = "t_user_details_info"]
pub struct UserDetailsInfo {
  pub f_mobile: String,
  pub f_name: Option<String>,
  pub f_annual_leave_days: f32,
  pub f_rest_days: f32,
  pub f_datetime: String,
  pub f_remark: Option<String>,
}

impl UserDetailsInfo {
  pub fn default() -> UserDetailsInfo {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let datetime: String = format!("{}-{:02}-{:02}", year, month, day);
    UserDetailsInfo {
      f_mobile: "".to_owned(),
      f_name: Some("".to_owned()),
      f_annual_leave_days: 0.0,
      f_rest_days: 0.0,
      f_datetime: datetime,
      f_remark: Some("".to_owned()),
    }
  }

  pub fn all_user_details_info(conn: &DbConn) -> QueryResult<Vec<UserDetailsInfo>> {
    use crate::schema::t_user_details_info::dsl::t_user_details_info as users;
    let data: Vec<UserDetailsInfo> = users
      .load::<UserDetailsInfo>(&conn.0)
      .expect("all_user_details_info get error");
    Ok(data)
  }

  pub fn get_user_details_by_mobile(
    conn: &DbConn,
    mobile: &String,
  ) -> QueryResult<Vec<UserDetailsInfo>> {
    use crate::schema::t_user_details_info::dsl::*;
    let data = t_user_details_info
      .filter(f_mobile.eq(mobile))
      .order(f_datetime.desc())
      .load::<UserDetailsInfo>(&conn.0);
    data
  }

  pub fn get_lastest_user_details_by_mobile(
    conn: &DbConn,
    mobile: &String,
  ) -> QueryResult<UserDetailsInfo> {
    let list = UserDetailsInfo::get_user_details_by_mobile(conn, mobile);
    match list {
      Ok(data) => {
        let latest_user_opt = data.get(0);
        match latest_user_opt {
          Some(latest_user) => Ok(latest_user.clone()),
          None => Err(diesel::result::Error::NotFound),
        }
      }
      _ => Err(diesel::result::Error::NotFound),
    }
  }

  pub fn insert_user_details(conn: &DbConn, user: UserDetailsInfo) -> QueryResult<usize> {
    use crate::schema::t_user_details_info::dsl::*;
    let rows_inserted = diesel::insert_into(t_user_details_info)
      .values((
        f_mobile.eq(user.f_mobile),
        f_name.eq(user.f_name),
        f_annual_leave_days.eq(user.f_annual_leave_days),
        f_rest_days.eq(user.f_rest_days),
        f_datetime.eq(user.f_datetime),
        f_remark.eq(user.f_remark),
      ))
      .execute(&conn.0);
    rows_inserted
  }

  pub fn update_user_detail(conn: &DbConn, user: UserDetailsInfo) -> QueryResult<usize> {
    use crate::schema::t_user_details_info::dsl::*;

    let rows_updated =
      diesel::update(t_user_details_info.find((user.f_mobile.clone(), user.f_datetime.clone())))
        .set((
          f_mobile.eq(user.f_mobile),
          f_name.eq(user.f_name),
          f_annual_leave_days.eq(user.f_annual_leave_days),
          f_rest_days.eq(user.f_rest_days),
          f_datetime.eq(user.f_datetime),
          f_remark.eq(user.f_remark),
        ))
        .execute(&conn.0);
    rows_updated
  }
}
