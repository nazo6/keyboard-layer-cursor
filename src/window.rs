use std::{num::NonZeroU32, sync::Arc};

use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, LogicalSize},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    platform::windows::WindowAttributesExtWindows,
    window::{Window, WindowId},
};

use crate::Config;

pub struct App {
    show: Option<(u8, u8, u8)>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    config: Config,
}

pub enum CustomEventLoopEvent {
    Redraw,
    SetColor(u8, u8, u8),
    SetPos(i32, i32),
    SetHide,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            show: None,
            surface: None,
            config,
        }
    }
}

impl ApplicationHandler<CustomEventLoopEvent> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attr = Window::default_attributes()
            .with_inner_size(LogicalSize::new(self.config.size.0, self.config.size.1))
            .with_transparent(false)
            .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
            .with_visible(self.show.is_some())
            .with_skip_taskbar(true)
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
                    s.window().set_visible(true);
                    self.show = Some((r, g, b));
                }
                CustomEventLoopEvent::SetPos(x, y) => {
                    s.window().set_outer_position(LogicalPosition::new(
                        x + self.config.offset.0,
                        y + self.config.offset.1,
                    ));
                }
                CustomEventLoopEvent::SetHide => {
                    s.window().set_visible(false);
                    self.show = None;
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
                if let (Some(surface), Some(color)) = (self.surface.as_mut(), self.show) {
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
                            buffer[index as usize] =
                                color.2 as u32 | ((color.1 as u32) << 8) | ((color.0 as u32) << 16);
                        }

                        buffer.present().unwrap();
                    }
                }
            }
            _ => (),
        }
    }
}

// fn get_hwnd(window: &Window) -> Result<HWND, winit::raw_window_handle::HandleError> {
//     let handle = window.window_handle()?;
//     if let RawWindowHandle::Win32(handle) = handle.as_raw() {
//         let hwnd = handle.hwnd;
//         Ok(unsafe { Handle::from_ptr(hwnd.get() as *mut c_void) })
//     } else {
//         panic!("Unsupported platform");
//     }
// }
