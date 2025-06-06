#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::LazyLock;

use tokio::runtime::Builder;
use window::CustomEventLoopEvent;
use winit::event_loop::EventLoop;

mod hid;
mod mouse_hook;
mod window;

#[derive(Debug, Clone)]
struct Config {
    layers: Vec<LayerConfig>,
    size: (u32, u32),
    offset: (i32, i32),
}

#[derive(Debug, Clone)]
enum LayerConfig {
    Color(u8, u8, u8),
    None,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    layers: vec![
        LayerConfig::None,
        LayerConfig::Color(0, 255, 0),
        LayerConfig::Color(255, 0, 0),
    ],
    size: (5, 10),
    offset: (20, 5),
});

fn main() {
    #[cfg(not(debug_assertions))]
    {
        // If program is executed in console, attach to it
        unsafe {
            let _ = windows::Win32::System::Console::AttachConsole(u32::MAX);
        }
    }

    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
    let mut app = window::App::new(CONFIG.clone());
    let event_loop = EventLoop::<CustomEventLoopEvent>::with_user_event()
        .build()
        .unwrap();
    let proxy = event_loop.create_proxy();

    runtime.block_on(async {
        runtime.spawn(hid::hid_task(proxy.clone(), CONFIG.layers.clone()));
        runtime.spawn(mouse_hook::mouse_hook_task(proxy));
        runtime.spawn(async move {
            let _ = tokio::signal::ctrl_c().await;
            std::process::exit(0);
        });
    });

    event_loop.run_app(&mut app).unwrap();
}
