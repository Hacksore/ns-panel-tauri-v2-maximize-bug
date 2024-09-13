#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_nspanel::{panel_delegate, WebviewWindowExt};

pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_nspanel::init())
    .setup(|app| {
      init(app.app_handle());

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn init(app_handle: &AppHandle) {
  let window: WebviewWindow = app_handle.get_webview_window("main").unwrap();

  let panel = window.to_panel().unwrap();

  #[allow(non_upper_case_globals)]
  pub const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

  // BUG: this is what's causing the crash because it allows double clicking somehow
  // which is not allowed on an nspanel
  #[allow(non_upper_case_globals)]
  const NSResizableWindowMask: i32 = 1 << 3;

  panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel + NSResizableWindowMask);

  let delegate = panel_delegate!(MyPanelDelegate {
    window_did_become_key,
    window_did_resign_key
  });

  let handle = app_handle.to_owned();

  delegate.set_listener(Box::new(move |delegate_name: String| {
    match delegate_name.as_str() {
      "window_did_become_key" => {
        let app_name = handle.package_info().name.to_owned();

        println!("[info]: {:?} panel becomes key window!", app_name);
      }
      "window_did_resign_key" => {
        println!("[info]: panel resigned from key window!");
      }
      _ => (),
    }
  }));

  panel.set_delegate(delegate);
}
