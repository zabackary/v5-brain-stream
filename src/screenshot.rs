use std::time::Duration;

use iced::futures::SinkExt;
use iced::{futures::Stream, stream};
use image::{GenericImageView, RgbaImage};
use tokio::task::spawn_blocking;
use vex_v5_serial::commands::file::DownloadFile;
use vex_v5_serial::connection::serial::{self};
use vex_v5_serial::connection::Connection;
use vex_v5_serial::packets::capture::{ScreenCapturePacket, ScreenCaptureReplyPacket};
use vex_v5_serial::packets::file::{FileTransferTarget, FileVendor};
use vex_v5_serial::string::FixedString;

use crate::{V5_BRAIN_RAW_SCREEN_SIZE, V5_BRAIN_SCREEN_SIZE};

pub fn screenshot_stream() -> impl Stream<Item = iced::widget::image::Handle> {
    stream::channel(10, |mut output| async move {
        // Find all vex devices on serial ports.
        let devices = serial::find_devices().expect("failed to enumerate devices");

        // Open a connection to the device.
        let mut connection = spawn_blocking(move || {
            devices
                .first()
                .expect("no device")
                .connect(Duration::from_secs(5))
                .expect("connection timeout")
        })
        .await
        .unwrap();

        loop {
            // Stolen from https://github.com/vexide/cargo-v5/blob/667a90226a1400613ac1a75c0c3191974d0a1f32/src/commands/screenshot.rs#L26

            // Tell the brain we want to take a screenshot
            if let Err(_) = connection
                .packet_handshake::<ScreenCaptureReplyPacket>(
                    Duration::from_millis(100),
                    5,
                    ScreenCapturePacket::new(()),
                )
                .await
            {
                break;
            }

            // Grab the image data
            let cap = connection
                .execute_command(DownloadFile {
                    file_name: FixedString::new("screen".to_string()).unwrap(),
                    vendor: FileVendor::Sys,
                    target: Some(FileTransferTarget::Cbuf),
                    load_addr: 0,
                    size: 512 * 272 * 4,
                    progress_callback: None,
                })
                .await
                .expect("couldn't download image from serial");

            let colors = cap
                .chunks(4)
                .filter_map(|p| {
                    if p.len() == 4 {
                        // little endian
                        let color = [p[2], p[1], p[0], 255];
                        Some(color)
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<_>>();

            let image = image::RgbaImage::from_vec(
                V5_BRAIN_RAW_SCREEN_SIZE.0,
                V5_BRAIN_RAW_SCREEN_SIZE.1,
                colors,
            )
            .unwrap();

            let image =
                RgbaImage::view(&image, 0, 0, V5_BRAIN_SCREEN_SIZE.0, V5_BRAIN_SCREEN_SIZE.1)
                    .to_image();

            output
                .send(iced::widget::image::Handle::from_rgba(
                    image.width(),
                    image.height(),
                    image.into_raw(),
                ))
                .await
                .expect("couldn't send image back");
        }
    })
}
