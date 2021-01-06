use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::FileField;
use rocket_multipart_form_data::{
  MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};

pub fn extract_files(
  file_name: &str,
  field: &std::sync::Arc<str>,
  content_type: &ContentType,
  data: Data,
) -> Result<Vec<FileField>, String> {
  let options =
    MultipartFormDataOptions::with_multipart_form_data_fields(vec![MultipartFormDataField::file(
      file_name,
    )
    .size_limit(32 * 1024 * 1024)]); // 32MB
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
  let files: Vec<FileField> = multipart_form_data.files.remove(field).unwrap_or(vec![]);
  Ok(files)
}
