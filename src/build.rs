use std::{env::{set_current_dir, current_dir}, path::Path, fs, array};

use json::array;
use tauri_cli;

use crate::consts::{TEMP_DIR, APPNAME};

pub fn build(name: String, url: String, icon_path: String) {
    let cwd = current_dir().unwrap();
    let wdPath = TEMP_DIR.as_ref().to_owned();
    let wd = wdPath.to_str().unwrap();
    tauri_cli::run(["init", "-A", &name, "--ci", "-d", wd, "-W", &name, "-l", "-D", &url].into_iter(), Some("tauri".to_string()));

    // update JSON config
    let appJsonPath =  Path::new(wd).join("src-tauri/tauri.conf.json");
    let appJson = fs::read_to_string(appJsonPath.clone()).expect("File should be opened");
    let mut appJsonObject = json::parse(&appJson).unwrap();

    appJsonObject["tauri"]["bundle"]["identifier"] = APPNAME.into();
    appJsonObject["tauri"]["bundle"]["icon"] = array![icon_path];

    write_to_file::write_to_file(appJsonPath.clone(), json::stringify(appJsonObject));


    // update main.rs to patch some code
    let main_rs = format!("{}{}{}", r#"
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
      .build()?;
    Ok(())
  })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
"#);

    let mainRsPath =  Path::new(wd).join("src-tauri/src/main.rs");

    write_to_file::write_to_file(mainRsPath.clone(), main_rs);

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

    let plistPath =  Path::new(wd).join("src-tauri/Info.plist");
    write_to_file::write_to_file(plistPath.clone(), plist);

    // build

    set_current_dir(wd);
    tauri_cli::run(["build"], Some("tauri".to_string()));


    let openPath = Path::new(wd).join("./src-tauri/target/release/bundle");

    open::that(openPath.clone());
}