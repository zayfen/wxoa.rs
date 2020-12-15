extern crate actix_rt;
extern crate tokio_test;

macro_rules! arun {
  ($e:expr) => {
    tokio_test::block_on($e)
  };
}

// #[actix_rt::test]
// async fn test_send_sms() {
//   use sms_service::send_sms;
//   send_sms("15927459238").await;
// }

#[test]
fn test_send_sms() {
  use sms_service::send_sms;
  send_sms("15927459238".to_owned());
}

// #[test]
// fn test_check_auth_code() {
//   use sms_service::check_auth_code;
//   assert_eq!(Ok(true), check_auth_code("15927459238", "711874"));
// }
