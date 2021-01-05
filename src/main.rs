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
mod models;
mod schema;
mod wx_utils;

use crate::models::UserDetailsInfo;
use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};
use commands_handlers::dispatch_command;
use rocket::http::ContentType;
use rocket::request::Form;
use rocket::Data;
use rocket_contrib::databases::diesel as rocket_diesel;
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
  MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};
use std::io::BufReader;
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
fn upload(content_type: &ContentType, data: Data) -> Result<String, String> {
  dbg!(content_type);
  let options =
    MultipartFormDataOptions::with_multipart_form_data_fields(vec![MultipartFormDataField::file(
      "excel",
    )
    .size_limit(32 * 1024 * 1024)]);
  dbg!("on upload post");
  dbg!(&options);
  let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
    Ok(multipart_form_data) => multipart_form_data,
    Err(err) => match err {
      MultipartFormDataError::DataTooLargeError(_) => {
        return Err("The file is too large".to_owned());
      }
      MultipartFormDataError::DataTypeError(_) => {
        dbg!(err);
        return Err("The file is not an excel.".to_owned());
      }
      _ => panic!("{:?}", err),
    },
  };

  let excel = multipart_form_data.files.get("excel");
  match excel {
    Some(mut excels) => {
      let file_excel = &excels[0];
      let path = &file_excel.path;
      dbg!(path);
      let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
      if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        let mut iter = RangeDeserializerBuilder::new()
          .from_range(&range)
          .expect(&"parse excel error".to_owned());
        while let Some(r) = iter.next() {
          let (id, name, mobile, day1, day2, date): (i32, String, String, f64, f64, String) =
            r.expect("parse excel error");
          println!(
            "{} :: {} ::  {} ::  {} :: {} :: {}",
            id, name, mobile, day1, day2, date
          );
        }
      }
      Ok("success".to_owned())
    }
    None => Err("empty file".to_owned()),
  }
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

fn main() {
  println!("Hello, world!");
  rocket::ignite()
    .attach(DbConn::fairing())
    .attach(Template::fairing())
    .mount("/", routes![index, validate, handle_message, upload])
    .launch();
}
