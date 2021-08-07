use std::path::PathBuf;


pub fn expected_header(entry: &PathBuf) -> (bool, Option<String>) {
  match entry.extension().unwrap().to_str().unwrap() {
    "l3" => {
      let mut header_file = entry.file_stem().unwrap().to_os_string();
      header_file.push(".h0");
      let header_file = entry.with_file_name(header_file);

      if header_file.exists() {
        (true, Some(header_file.to_str().unwrap().to_string()))
      } else {
        (false, Some("../runtime/15411-l3.h0".to_string()))
      }
    },

    "l4" => (false, Some("../runtime/15411-l4.h0".to_string())),
    _ => (false, None),
  }
}
