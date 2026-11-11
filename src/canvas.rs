use eframe::egui;

pub struct Canvas {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Canvas {
    pub const CW: u32 = 1024;
    pub const CH: u32 = 768;

    pub fn new() -> Self {
        let data = vec![255u8; (Self::CW * Self::CH * 4) as usize];
        Self { data, width: Self::CW, height: Self::CH }
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn data(&self) -> &[u8] { &self.data }
    pub fn data_mut(&mut self) -> &mut [u8] { &mut self.data }

    /// Get pixel color at (x, y), or None if out of bounds.
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<egui::Color32> {
        if x < self.width && y < self.height {
            let i = ((y * self.width + x) * 4) as usize;
            Some(egui::Color32::from_rgba_unmultiplied(
                self.data[i], self.data[i+1], self.data[i+2], self.data[i+3]
            ))
        } else {
            None
        }
    }

    /// Set pixel color at (x, y), clamped to canvas bounds.
    pub fn set_pixel(&mut self, x: u32, y: u32, c: egui::Color32) {
        let x = x.min(self.width.saturating_sub(1)).max(0);
        let y = y.min(self.height.saturating_sub(1)).max(0);
        let i = ((y * self.width + x) * 4) as usize;
        let rgba = c.to_srgba_unmultiplied();
        self.data[i]     = rgba[0];
        self.data[i + 1] = rgba[1];
        self.data[i + 2] = rgba[2];
        self.data[i + 3] = rgba[3];
    }

    /// Set pixel color without clamping (caller must ensure bounds).
    #[inline]
    pub fn set_pixel_unchecked(&mut self, x: u32, y: u32, c: egui::Color32) {
        let i = ((y * self.width + x) * 4) as usize;
        let rgba = c.to_srgba_unmultiplied();
        self.data[i]     = rgba[0];
        self.data[i + 1] = rgba[1];
        self.data[i + 2] = rgba[2];
        self.data[i + 3] = rgba[3];
    }

    pub fn clone_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn load_from_pixels(&mut self, pixels: &[u8]) {
        let needed = (self.width * self.height * 4) as usize;
        if pixels.len() >= needed {
            self.data[..needed].copy_from_slice(&pixels[..needed]);
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
