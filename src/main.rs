#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate calamine;
extern crate regex;
extern crate rocket_multipart_form_data;
extern crate sms_service;

mod commands_handlers;
mod datetime_utils;
mod excel_utils;
mod models;
mod multipart_utils;
mod schema;
mod wx_utils;

use crate::models::UserDetailsInfo;
use commands_handlers::dispatch_command;
use excel_utils::extract_rows;
use multipart_utils::extract_files;
use rocket::http::ContentType;
use rocket::request::Form;
use rocket::Data;
use rocket_contrib::databases::diesel as rocket_diesel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use wx_utils::parse_xml_msg::{build_text_msg, parse_msg};

#[database("bottle_zoa")]
pub struct DbConn(rocket_diesel::MysqlConnection);

#[derive(serde::Serialize)]
pub struct Noop {}

pub struct ExcelRow(String, String, String, String, String, String);

#[get("/")]
fn index(conn: DbConn) -> Template {
  let data = UserDetailsInfo::all_user_details_info(&conn);
  println!("all users info: {:?}", &data);
  if data.is_ok() {
    Template::render("index", &data.unwrap())
  } else {
    Template::render("index", &Noop {})
  }
}

#[post("/upload", data = "<data>")]
fn upload(conn: DbConn, content_type: &ContentType, data: Data) -> Result<String, String> {
  dbg!(content_type);
  let files = extract_files("excel", &std::sync::Arc::from("excel"), content_type, data)
    .expect("no upload files");
  let file_excel = &files[0];
  let path = &file_excel.path;
  // (姓名，手机号，年假，调休，统计日期，备注)
  type RowType = (String, String, f64, f64, String, String);
  let rows: Vec<RowType> = extract_rows(path).expect("extract_rows error");
  for row in &rows {
    match UserDetailsInfo::insert_user_details(
      &conn,
      UserDetailsInfo {
        f_name: Some(row.0.clone()),
        f_mobile: row.1.clone(),
        f_annual_leave_days: row.2 as f32,
        f_rest_days: row.3 as f32,
        f_datetime: row.4.clone(),
        f_remark: Some(row.5.clone()),
      },
    ) {
      Ok(_) => {}
      Err(_e) => {}
    }
  }

  Ok(format!(
    "{:?}",
    rows
      .into_iter()
      .map(|v| UserDetailsInfo {
        f_name: Some(v.0),
        f_mobile: v.1,
        f_annual_leave_days: v.2 as f32,
        f_rest_days: v.3 as f32,
        f_datetime: v.4,
        f_remark: Some(v.5),
      })
      .collect::<Vec<UserDetailsInfo>>()
  ))
}

#[derive(Debug, Clone, FromForm)]
struct CallbackQuery {
  signature: String,
  echostr: String,
  timestamp: u64,
  nonce: String,
}

#[get("/callback?<fields..>")]
fn validate(fields: Form<CallbackQuery>) -> String {
  println!("query: {:?}", &fields);
  fields.echostr.clone()
}

#[post("/callback", format = "text/xml", data = "<msg>")]
fn handle_message(conn: DbConn, msg: rocket::Data) -> String {
  // println!("msg from wx: {:?}", msg);
  let msg = std::str::from_utf8(msg.peek()).unwrap().to_owned();
  if let Some(map) = parse_msg(&msg) {
    let msg_type: String = map["MsgType"].clone();
    let mut response_msg: String = "".to_owned();

    // 订阅，取消订阅 事件
    if map.contains_key("Event") {
      if map["Event"] == "subscribe" {
        // TODO: 订阅
      }

      if map["Event"] == "unsubscribe" {
        // TODO: 取消订阅
      }
    }

    // 文字信息
    let command: String = map["Content"].clone();
    if msg_type == "text" {
      response_msg = dispatch_command(&conn, &map, &command);
    }

    // 回复消息
    let response_msg = build_text_msg(
      &map["ToUserName"],
      &map["FromUserName"],
      response_msg.to_owned(),
    );
    println!("response_msg: {:?}", &response_msg);
    return response_msg;
  } else {
    return "error".to_owned();
  }
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
  format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
  println!("Hello, world!");
  rocket::ignite()
    .attach(DbConn::fairing())
    .attach(Template::fairing())
    .mount("/", routes![index, validate, handle_message, upload])
    .mount(
      "/",
      StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
    )
    .register(catchers![not_found])
    .launch();
}
