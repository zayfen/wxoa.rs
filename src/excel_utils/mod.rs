use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use std::path::Path;

pub fn extract_rows<'a, T, P>(path: P) -> Result<Vec<T>, String>
where
  T: for<'de> serde::Deserialize<'de>,
  P: AsRef<Path>,
{
  let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
  let mut rows: Vec<T> = vec![];
  if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
    let mut iter = RangeDeserializerBuilder::new()
      .from_range(&range)
      .expect(&"parse excel error".to_owned());
    while let Some(r) = iter.next() {
      let rr: T = r.expect("parse excel error");
      rows.push(rr);
    }
  }
  Ok(rows)
}
