use cocoa::appkit::NSWindowCollectionBehavior;
use tauri::{Manager, Runtime, WebviewWindow};
use tauri_nspanel::WebviewWindowExt;

pub trait WebviewWindowExtMacos {
  fn set_float_panel(&self, level: i32);
}

impl<R: Runtime> WebviewWindowExtMacos for WebviewWindow<R> {
  fn set_float_panel(&self, level: i32) {
    let panel = self.to_panel().unwrap();

    panel.set_level(level);

    #[allow(non_upper_case_globals)]
    const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

    #[allow(non_upper_case_globals)]
    const NSResizableWindowMask: i32 = 1 << 3;

    panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel + NSResizableWindowMask);

    panel.set_collection_behaviour(
      NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
    );
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(move |app| {
      let window = app.get_webview_window("main").unwrap();
      window.set_float_panel(1);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
