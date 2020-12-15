#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate sms_service;

mod commands_handlers;
mod datetime_utils;
mod models;
mod schema;
mod wx_utils;

use crate::models::UserInfo;
use commands_handlers::dispatch_command;
use rocket::request::Form;
use rocket_contrib::databases::diesel as rocket_diesel;
use rocket_contrib::templates::Template;
use wx_utils::parse_xml_msg::{build_text_msg, parse_msg};

#[database("bottle_zoa")]
pub struct DbConn(rocket_diesel::MysqlConnection);

#[derive(serde::Serialize)]
pub struct Noop {}

#[get("/")]
fn index(conn: DbConn) -> Template {
  let data = UserInfo::all_user_info(&conn);
  println!("all users info: {:?}", &data);
  if data.is_ok() {
    Template::render("index", &data.unwrap())
  } else {
    Template::render("index", &Noop {})
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
    .mount("/", routes![index, validate, handle_message])
    .launch();
}
