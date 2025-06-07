use std::time::Duration;

use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_LOCATIONCHANGE;
use wineventhook::{EventFilter, MaybeKnown, ObjectWindowEvent, WindowEventHook, WindowEventType};
use winit::event_loop::EventLoopProxy;

use crate::window::CustomEventLoopEvent;

pub async fn mouse_hook_task(proxy: EventLoopProxy<CustomEventLoopEvent>) -> anyhow::Result<()> {
    let (mouse_event_tx, mut mouse_event_rx) = tokio::sync::mpsc::unbounded_channel();

    let _hook = WindowEventHook::hook(
        EventFilter::default().event(EVENT_OBJECT_LOCATIONCHANGE as i32),
        mouse_event_tx,
    )
    .await?;

    loop {
        let mut pos_change = false;
        let task_pos_change = async {
            loop {
                if let Some(mouse_ev) = mouse_event_rx.recv().await {
                    if let WindowEventType::Object(MaybeKnown::Known(
                        ObjectWindowEvent::LocationChange,
                    )) = mouse_ev.event_type()
                    {
                        pos_change = true;
                    }
                }
            }
        };
        let task_throttle_timer = async {
            tokio::time::sleep(Duration::from_millis(10)).await;
        };

        tokio::select!(_ = task_pos_change => {}, _ = task_throttle_timer => {});

        if pos_change {
            let mut pos = unsafe { std::mem::zeroed() };
            let res = unsafe { windows::Win32::UI::WindowsAndMessaging::GetCursorPos(&mut pos) };

            if res.is_ok() {
                let _ = proxy.send_event(CustomEventLoopEvent::SetPos(pos.x, pos.y));
            }
        }
    }
}
