use chrono::{prelude::*, Utc};

pub fn utc_now_date() -> (i32, u32, u32) {
  let now = Utc::now();
  let year = now.year();
  let month = now.month();
  let day = now.day();
  (year, month, day)
}

pub fn now_date() -> (i32, u32, u32) {
  let dt = Utc::now();
  let beijing_dt = dt.with_timezone(&FixedOffset::east(8 * 3600));
  let year: i32 = beijing_dt.year();
  let month: u32 = beijing_dt.month();
  let day: u32 = beijing_dt.day();
  (year, month, day)
}
