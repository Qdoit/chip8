use crate::app::ColorConfig;
use pixels::Pixels;

pub struct Display(pub Pixels, pub ColorConfig);

impl Display {
    pub fn set_pixel(&mut self, x: u8, y: u8) {
        let fb = self.0.frame_mut();
        let x = x as usize;
        let y = y as usize;
        fb[(y * 64 + x) * 4] = self.1.fg_on_color.0;
        fb[(y * 64 + x) * 4 + 1] = self.1.fg_on_color.1;
        fb[(y * 64 + x) * 4 + 2] = self.1.fg_on_color.2;
        fb[(y * 64 + x) * 4 + 3] = 255;
    }

    pub fn unset_pixel(&mut self, x: u8, y: u8) {
        let fb = self.0.frame_mut();
        let x = x as usize;
        let y = y as usize;
        fb[(y * 64 + x) * 4] = self.1.fg_off_color.0;
        fb[(y * 64 + x) * 4 + 1] = self.1.fg_off_color.1;
        fb[(y * 64 + x) * 4 + 2] = self.1.fg_off_color.2;
        fb[(y * 64 + x) * 4 + 3] = 255;
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        let fb = self.0.frame();
        let (x, y) = (x as usize, y as usize);
        !(fb[(y * 64 + x) * 4] == self.1.fg_off_color.0
            && fb[(y * 64 + x) * 4 + 1] == self.1.fg_off_color.1
            && fb[(y * 64 + x) * 4 + 2] == self.1.fg_off_color.2)
    }

    pub fn clear_screen(&mut self) {
        let fb = self.0.frame_mut();
        for i in 0..fb.len() {
            if (i + 1) % 4 == 0 {
                fb[i] = 255;
            } else if i % 4 == 0 {
                fb[i] = self.1.fg_off_color.0;
            } else if (i + 3) % 4 == 0 {
                fb[i] = self.1.fg_off_color.1;
            } else {
                fb[i] = self.1.fg_off_color.2;
            }
        }
    }
}
