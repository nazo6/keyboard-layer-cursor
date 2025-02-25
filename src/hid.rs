use async_hid::{AccessMode, DeviceInfo};
use futures::StreamExt as _;
use winit::event_loop::EventLoopProxy;

use crate::window::CustomEventLoopEvent;

pub async fn hid_task(proxy: EventLoopProxy<CustomEventLoopEvent>) -> anyhow::Result<()> {
    let mut devices = DeviceInfo::enumerate().await?;
    let mut device = None;
    while let Some(info) = devices.next().await {
        if info.usage_page == 0xFF60 && info.usage_id == 0x61 {
            device = Some(info);
        }
    }

    let Some(device) = device else {
        return Err(anyhow::anyhow!("No device found"));
    };

    let device = device.open(AccessMode::ReadWrite).await?;

    let _ = proxy.send_event(CustomEventLoopEvent::SetColor(255, 0, 0));

    loop {
        let mut buf = [0; 32];
        device.read_input_report(&mut buf).await?;
        if buf[0] == 0x01 {
            println!("Layer: {}", buf[1]);
            let color = match buf[1] {
                0 => (0, 0, 255),
                1 => (0, 255, 0),
                2 => (255, 0, 0),
                _ => {
                    continue;
                }
            };
            let _ = proxy.send_event(CustomEventLoopEvent::SetColor(color.0, color.1, color.2));
        }
    }
}
