# ns-panel-tauri-v2-maximize-bug

https://github.com/user-attachments/assets/bd2d025d-6d10-44ae-b45a-79d87a66ff54

stack
```

> ns-panel-tauri-v2-maximize-bug@0.1.0 tauri /Users/hacksore/code/repro/ns-panel-tauri-v2-maximize-bug
> tauri "dev"

    Running BeforeDevCommand (`pnpm dev`)

> ns-panel-tauri-v2-maximize-bug@0.1.0 dev /Users/hacksore/code/repro/ns-panel-tauri-v2-maximize-bug
> vite


  VITE v5.4.4  ready in 85 ms

  ➜  Local:   http://localhost:1420/
    Info Watching /Users/hacksore/code/repro/ns-panel-tauri-v2-maximize-bug/src-tauri for changes...
[1m[32m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
[info]: "ns-panel-tauri-v2-maximize-bug" panel becomes key window!
thread 'main' panicked at library/core/src/panicking.rs:221:5:
panic in a function that cannot unwind
stack backtrace:
   0: rust_begin_unwind
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/std/src/panicking.rs:665:5
   1: core::panicking::panic_nounwind_fmt::runtime
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/core/src/panicking.rs:112:18
   2: core::panicking::panic_nounwind_fmt
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/core/src/panicking.rs:122:5
   3: core::panicking::panic_nounwind
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/core/src/panicking.rs:221:5
   4: core::panicking::panic_cannot_unwind
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/core/src/panicking.rs:309:5
   5: objc_exception::try_no_ret::try_objc_execute_closure
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:30:5
   6: RustObjCExceptionTryCatch
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/extern/exception.m:10:9
   7: objc_exception::try_no_ret
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:44:19
   8: objc_exception::try
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:67:9
   9: objc::exception::try
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/exception.rs:8:5
  10: objc::message::platform::send_unverified
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:12:9
  11: objc::message::send_message
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:178:5
  12: <*mut objc::runtime::Object as cocoa::appkit::NSWindow>::setStyleMask_
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cocoa-0.26.0/src/appkit.rs:1405:9
  13: tao::platform_impl::platform::util::async::set_style_mask
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/util/async.rs:55:3
  14: tao::platform_impl::platform::util::async::set_style_mask_sync
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/util/async.rs:74:5
  15: tao::platform_impl::platform::window::UnownedWindow::set_style_mask_sync
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/window.rs:592:14
  16: tao::platform_impl::platform::window::UnownedWindow::is_zoomed
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/window.rs:899:7
  17: tao::platform_impl::platform::window::UnownedWindow::is_maximized
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/window.rs:992:5
  18: tao::window::Window::is_maximized
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/window.rs:897:5
  19: tauri_runtime_wry::handle_user_message
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-runtime-wry-2.0.0-rc.10/src/lib.rs:2748:53
  20: tauri_runtime_wry::handle_event_loop
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-runtime-wry-2.0.0-rc.10/src/lib.rs:3584:9
  21: <tauri_runtime_wry::Wry<T> as tauri_runtime::Runtime<T>>::run::{{closure}}
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-runtime-wry-2.0.0-rc.10/src/lib.rs:2653:7
  22: <tao::platform_impl::platform::app_state::EventLoopHandler<T> as tao::platform_impl::platform::app_state::EventHandler>::handle_user_events::{{closure}}
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/app_state.rs:117:11
  23: tao::platform_impl::platform::app_state::EventLoopHandler<T>::with_callback
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/app_state.rs:79:7
  24: <tao::platform_impl::platform::app_state::EventLoopHandler<T> as tao::platform_impl::platform::app_state::EventHandler>::handle_user_events
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/app_state.rs:111:5
  25: tao::platform_impl::platform::app_state::Handler::handle_user_events
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/app_state.rs:217:7
  26: tao::platform_impl::platform::app_state::AppState::cleared
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/app_state.rs:390:5
  27: tao::platform_impl::platform::observer::control_flow_end_handler::{{closure}}
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/observer.rs:187:11
  28: tao::platform_impl::platform::observer::control_flow_handler::{{closure}}
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/observer.rs:148:5
  29: std::panicking::try::do_call
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/std/src/panicking.rs:557:40
  30: ___rust_try
  31: std::panicking::try
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/std/src/panicking.rs:521:19
  32: std::panic::catch_unwind
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/std/src/panic.rs:350:14
  33: tao::platform_impl::platform::event_loop::stop_app_on_panic
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/event_loop.rs:261:9
  34: tao::platform_impl::platform::observer::control_flow_handler
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/observer.rs:146:3
  35: tao::platform_impl::platform::observer::control_flow_end_handler
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/observer.rs:182:5
  36: <unknown>
  37: <unknown>
  38: <unknown>
  39: <unknown>
  40: <unknown>
  41: <unknown>
  42: <unknown>
  43: <unknown>
  44: <unknown>
  45: <unknown>
  46: <() as objc::message::MessageArguments>::invoke
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:128:17
  47: objc::message::platform::send_unverified::{{closure}}
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/apple/mod.rs:27:9
  48: objc_exception::try::{{closure}}
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:68:31
  49: objc_exception::try_no_ret::try_objc_execute_closure
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:34:9
  50: RustObjCExceptionTryCatch
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/extern/exception.m:10:9
  51: objc_exception::try_no_ret
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:44:19
  52: objc_exception::try
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:67:9
  53: objc::exception::try
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/exception.rs:8:5
  54: objc::message::platform::send_unverified
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:12:9
  55: objc::message::send_message
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:178:5
  56: tao::platform_impl::platform::event_loop::EventLoop<T>::run_return
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/event_loop.rs:218:16
  57: tao::platform_impl::platform::event_loop::EventLoop<T>::run
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/platform_impl/macos/event_loop.rs:185:21
  58: tao::event_loop::EventLoop<T>::run
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.30.0/src/event_loop.rs:211:5
  59: <tauri_runtime_wry::Wry<T> as tauri_runtime::Runtime<T>>::run
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-runtime-wry-2.0.0-rc.10/src/lib.rs:2633:5
  60: tauri::app::App<R>::run
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-2.0.0-rc.11/src/app.rs:1094:5
  61: tauri::app::Builder<R>::run
             at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-2.0.0-rc.11/src/app.rs:1957:5
  62: ns_panel_tauri_v2_maximize_bug_lib::run
             at ./src/lib.rs:10:3
  63: ns_panel_tauri_v2_maximize_bug::main
             at ./src/main.rs:5:3
  64: core::ops::function::FnOnce::call_once
             at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread caused non-unwinding panic. aborting.
 ELIFECYCLE  Command failed with exit code 1.
```


