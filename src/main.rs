use tokio::runtime::Builder;
use winit::event_loop::EventLoop;

mod hid;
mod mouse_hook;
mod window;

fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
    let mut app = window::App::new();
    let event_loop = EventLoop::<window::CustomEventLoopEvent>::with_user_event()
        .build()
        .unwrap();
    let proxy = event_loop.create_proxy();

    runtime.block_on(async {
        runtime.spawn(hid::hid_task(proxy.clone()));
        runtime.spawn(mouse_hook::mouse_hook_task(proxy));
    });

    event_loop.run_app(&mut app).unwrap();
}
