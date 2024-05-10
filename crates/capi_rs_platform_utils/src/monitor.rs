#[derive(Default)]
pub struct MonitorImageWrapper {
    width: usize,
    height: usize,
    img_bytes: Option<Vec<u8>>,
}

impl MonitorImageWrapper {
    pub fn new(width: usize, height: usize, img: Option<Vec<u8>>) -> Self {
        Self {
            width,
            height,
            img_bytes: img,
        }
    }
    pub fn write_to_file(&self, filename: &str) {
        if let Some(bytes) = &self.img_bytes {
            crate::utils::write_bytes_to_file(bytes.as_slice(), filename);
        }
    }
}
