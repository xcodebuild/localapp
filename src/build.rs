use std::{env::{set_current_dir, current_dir}, path::Path, fs, array};

use futures::StreamExt;
use json::array;
use tauri_cli;
use random_string::generate;


use crate::consts::{TEMP_DIR, APPNAME};
use std::fs::File;
use std::io::Write;


pub fn build(name: String, url: String, icon_path: String) {
    let cwd = current_dir().unwrap();
    let wd_path = TEMP_DIR.to_string();
    let wd = wd_path.as_str();
    tauri_cli::run(["init", "-A", &name, "--ci", "-d", wd, "-W", &name, "-l", "-D", &url].into_iter(), Some("tauri".to_string())).unwrap();

    // update JSON config
    let app_json_path =  Path::new(wd).join("src-tauri/tauri.conf.json");
    let app_json = fs::read_to_string(app_json_path.clone()).expect("App.json should be opened");
    let mut app_json_object = json::parse(&app_json).unwrap();

    app_json_object["tauri"]["windows"] = array![];
    app_json_object["tauri"]["windows"] = array![];
    let charset = "1234567890abcksdljfsdngkjsde";

    app_json_object["tauri"]["bundle"]["identifier"] = generate(10, charset).into();
    app_json_object["tauri"]["bundle"]["icon"] = array![icon_path];

    app_json_object["build"]["distDir"] = url.clone().into();
    app_json_object["build"]["devPath"] = url.clone().into();

    app_json_object["package"]["productName"] = name.clone().into();

    let mut app_json_output = File::create(app_json_path.clone()).unwrap();
    write!(app_json_output, "{}", json::stringify(app_json_object)).unwrap();


    // update main.rs to patch some code
    let main_rs = format!("{}{}{}{}{}", r#"
    #![cfg_attr(
        all(not(debug_assertions), target_os = "windows"),
        windows_subsystem = "windows"
      )]
      use tauri::{Menu, MenuItem, Submenu, WindowBuilder, WindowUrl};
      use tauri::Manager;
      
      fn main() {
          let about_menu = Submenu::new("App", Menu::new()
  .add_native_item(MenuItem::Hide)
  .add_native_item(MenuItem::HideOthers)
  .add_native_item(MenuItem::ShowAll)
  .add_native_item(MenuItem::Separator)
  .add_native_item(MenuItem::Quit));

let edit_menu = Submenu::new("Edit", Menu::new()
  .add_native_item(MenuItem::Undo)
  .add_native_item(MenuItem::Redo)
  .add_native_item(MenuItem::Separator)
  .add_native_item(MenuItem::Cut)
  .add_native_item(MenuItem::Copy)
  .add_native_item(MenuItem::Paste)
  .add_native_item(MenuItem::SelectAll));

let view_menu = Submenu::new("View", Menu::new()
  .add_native_item(MenuItem::EnterFullScreen));

let window_menu = Submenu::new("Window", Menu::new()
  .add_native_item(MenuItem::Minimize)
  .add_native_item(MenuItem::Zoom));

let menu = Menu::new()
  .add_submenu(about_menu)
  .add_submenu(edit_menu)
  .add_submenu(view_menu)
  .add_submenu(window_menu);

  tauri::Builder::default()
  .menu(menu)
  .setup(|app| {
    WindowBuilder::new(app, "core", WindowUrl::App(""#, url, r#"".into()))
      .initialization_script("\
      window.addEventListener('load', () => {\
        document.addEventListener('compositionend', function(e) {\
          e.preventDefault();\
          e.stopPropagation();\
        }, true);\
      })")
      .title(""#, name.clone(), r#"")
      .enable_clipboard_access()
      .inner_size(1200.0, 800.0)
      .build()?;
    Ok(())
  })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
"#);

    let main_rs_path =  Path::new(wd).join("src-tauri/src/main.rs");
    let mut main_rs_output = File::create(main_rs_path.clone()).unwrap();
    write!(main_rs_output, "{}", main_rs).unwrap();

    // write info.plist
    let plist = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
      <key>NSCameraUsageDescription</key>
      <string>Request camera access for WebRTC</string>
      <key>NSMicrophoneUsageDescription</key>
      <string>Request microphone access for WebRTC</string>
    </dict>
    </plist>
    "#;

    let plist_path =  Path::new(wd).join("src-tauri/Info.plist");
    let mut output = File::create(plist_path.clone()).unwrap();
    write!(output, "{}", plist).unwrap();

    // build

    set_current_dir(wd).unwrap();
    tauri_cli::run(["build"], Some("tauri".to_string())).unwrap();


    let open_path = Path::new(wd).join("./src-tauri/target/release/bundle");

    open::that(open_path.clone()).unwrap();
}