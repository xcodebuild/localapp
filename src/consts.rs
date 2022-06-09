
use tempdir::TempDir;
use once_cell::sync::Lazy;
use std::path::Path;
use std::fs;

pub const USER_AGENT_REQUEST: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.61 Safari/537.36";
pub const APPNAME: &str = "localapp";


pub static TEMP_DIR: Lazy<String> = Lazy::new(|| {
    let dir = TempDir::new(APPNAME);
    let path = Path::new(dir.unwrap().path()).join("../localapp-build-dir");
    let result = path.as_path().to_str().unwrap();
    fs::create_dir_all(result.clone());
    String::from(result)
});
