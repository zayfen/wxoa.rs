extern crate reqwest;
#[macro_use]
extern crate serde_json;
extern crate redis;
use redis::Commands;
use std::thread;

use reqwest::header::{HeaderMap, HeaderValue};

pub fn send_sms(mobile: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = "http://10.96.153.33:50051/v1/sms/send";

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("cache-control", HeaderValue::from_static("no-cache"));

    let body_data = json!({
      "phone": mobile,
      "platform": 0,
      "expiration": 300,
      "len": 6,
      "type": 1
    });

    let res = client.post(url).headers(headers).json(&body_data).send();
    println!("send_sms response: {:?}", &res);
    res
}

pub fn send_sms_async(mobile: String) -> std::thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        send_sms(mobile).unwrap();
    });

    handle
}

pub fn check_auth_code(mobile: &str, auth_code: &str) -> redis::RedisResult<bool> {
    let client = redis::Client::open("redis://10.96.153.218:6379/0")?;
    let mut conn = client.get_connection()?;
    let key = format!("verificationCode:backstage:verificationIdentity:{}", mobile);
    // let key = "verificationCode:backstage:verificationIdentity:count:13286661539";
    dbg!(&key);
    let real_auth_code: String = conn.get(key).unwrap_or("".to_owned());
    println!("real_auth_code: {:?}", &real_auth_code);
    dbg!(&real_auth_code);
    if real_auth_code == "" {
        return Err(redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "没有找到auth_code",
        )));
    }
    if real_auth_code == auth_code {
        Ok(true)
    } else {
        Err(redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "验证码错误",
        )))
    }
}
