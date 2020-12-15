table! {
  use diesel::sql_types::*;

  t_user_details_info (f_mobile, f_datetime) {
    f_mobile -> Varchar,
    f_name -> Nullable<Varchar>,
    f_annual_leave_days -> Float,
    f_rest_days -> Float,
    f_datetime -> Varchar,
    f_remark -> Nullable<Varchar>,
  }
}

table! {
  use diesel::sql_types::*;

  t_user_info (f_open_id) {
    f_open_id -> Varchar,
    f_mobile -> Varchar,
    f_state -> Integer,
    f_remark -> Nullable<Varchar>,
  }
}

allow_tables_to_appear_in_same_query!(t_user_details_info, t_user_info,);
