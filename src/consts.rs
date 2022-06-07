
use tempdir::TempDir;
use once_cell::sync::Lazy;

pub const USER_AGENT_REQUEST: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.61 Safari/537.36";
pub const APPNAME: &str = "localapp";


pub static TEMP_DIR: Lazy<TempDir> = Lazy::new(|| {
    let dir = TempDir::new(APPNAME);
    dir.unwrap()
});
