use async_hid::{AccessMode, DeviceInfo};
use futures::StreamExt as _;
use winit::event_loop::EventLoopProxy;

use crate::{LayerConfig, window::CustomEventLoopEvent};

pub async fn hid_task(
    proxy: EventLoopProxy<CustomEventLoopEvent>,
    config: Vec<LayerConfig>,
) -> anyhow::Result<()> {
    loop {
        if let Err(e) = handle_hid(&proxy, &config).await {
            println!("HID reconnecting: {:?}", e);
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    }
}

async fn handle_hid(
    proxy: &EventLoopProxy<CustomEventLoopEvent>,
    config: &Vec<LayerConfig>,
) -> anyhow::Result<()> {
    let layer_to_ev = |l: usize| -> Option<CustomEventLoopEvent> {
        config.get(l).map(|l| match l {
            LayerConfig::Color(r, g, b) => CustomEventLoopEvent::SetColor(*r, *g, *b),
            LayerConfig::None => CustomEventLoopEvent::SetHide,
        })
    };

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

    if let Some(ev) = layer_to_ev(0) {
        let _ = proxy.send_event(ev);
    }

    loop {
        let mut buf = [0; 32];
        device.read_input_report(&mut buf).await?;
        if buf[0] == 0x01 {
            println!("Layer: {}", buf[1]);
            if let Some(ev) = layer_to_ev(buf[1] as usize) {
                let _ = proxy.send_event(ev);
            }
        }
    }
}
