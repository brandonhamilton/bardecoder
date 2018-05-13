use super::locate::QRLocation;

use std::ops::Index;

use image::GrayImage;

pub trait Extract<T> {
    fn extract(&self, threshold: &T, locs: Vec<QRLocation>) -> Vec<QRData>;
}

#[derive(Debug)]
pub struct QRData {
    pub data: Vec<u8>,
    pub version: u32,

    pub side: u32,
}

impl QRData {
    pub fn new(data: Vec<u8>, version: u32) -> QRData {
        QRData {
            data,
            version,
            side: 4 * version + 17,
        }
    }
}

impl Index<(usize, usize)> for QRData {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &u8 {
        &self.data[index.0 * self.side as usize + index.1]
    }
}

pub struct QRExtractor {}

impl QRExtractor {
    pub fn new() -> QRExtractor {
        QRExtractor {}
    }
}

impl Extract<GrayImage> for QRExtractor {
    fn extract(&self, threshold: &GrayImage, locs: Vec<QRLocation>) -> Vec<QRData> {
        let mut qr_data = vec![];

        for loc in locs {
            let size = 17 + loc.version * 4;

            let mut dx = loc.top_right - loc.top_left;
            dx = dx / (size - 7) as f64;

            let mut dy = loc.bottom_left - loc.top_left;
            dy = dy / (size - 7) as f64;

            let mut start = loc.top_left - 3.0 * dx;
            start = start - 3.0 * dy;

            let mut data = vec![];

            for _ in 0..size {
                let mut line = start.clone();

                for _ in 0..size {
                    data.push(threshold.get_pixel(line.x.round() as u32, line.y.round() as u32)[0]);
                    line = line + dx;
                }

                start = start + dy;
            }

            qr_data.push(QRData::new(data, loc.version));
        }

        qr_data
    }
}
