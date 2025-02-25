use std::{ffi::c_void, num::NonZeroU32, sync::Arc};

use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, LogicalSize},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::{Window, WindowId},
};
use winsafe::{HWND, prelude::Handle};

pub struct App {
    color: (u8, u8, u8),
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
}

pub enum CustomEventLoopEvent {
    Redraw,
    SetColor(u8, u8, u8),
    SetPos(i32, i32),
}

impl App {
    pub fn new() -> Self {
        Self {
            color: (0, 0, 0),
            surface: None,
        }
    }
}

impl ApplicationHandler<CustomEventLoopEvent> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attr = Window::default_attributes()
            .with_inner_size(LogicalSize::new(5, 10))
            .with_transparent(false)
            .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
            .with_decorations(false);
        let window = Arc::new(event_loop.create_window(attr).unwrap());
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        self.surface = Some(surface);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: CustomEventLoopEvent) {
        if let Some(s) = self.surface.as_ref() {
            match event {
                CustomEventLoopEvent::Redraw => {}
                CustomEventLoopEvent::SetColor(r, g, b) => {
                    self.color = (r, g, b);
                }
                CustomEventLoopEvent::SetPos(x, y) => {
                    s.window()
                        .set_outer_position(LogicalPosition::new(x + 20, y + 5));
                }
            }

            s.window().request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(surface) = self.surface.as_mut() {
                    if surface.window().id() == id {
                        let (width, height) = {
                            let size = surface.window().inner_size();
                            (size.width, size.height)
                        };
                        surface
                            .resize(
                                NonZeroU32::new(width).unwrap(),
                                NonZeroU32::new(height).unwrap(),
                            )
                            .unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();
                        for index in 0..(width * height) {
                            buffer[index as usize] = self.color.2 as u32
                                | ((self.color.1 as u32) << 8)
                                | ((self.color.0 as u32) << 16);
                        }

                        buffer.present().unwrap();
                    }
                }
            }
            _ => (),
        }
    }
}

fn get_hwnd(window: &Window) -> Result<HWND, winit::raw_window_handle::HandleError> {
    let handle = window.window_handle()?;
    if let RawWindowHandle::Win32(handle) = handle.as_raw() {
        let hwnd = handle.hwnd;
        Ok(unsafe { Handle::from_ptr(hwnd.get() as *mut c_void) })
    } else {
        panic!("Unsupported platform");
    }
}
