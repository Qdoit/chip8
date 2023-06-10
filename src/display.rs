use pixels::Pixels;

pub struct Display(pub Pixels);

impl Display {
    pub fn set_pixel(&mut self, x: u8, y: u8) {
        let fb = self.0.frame_mut();
        let x = x as usize;
        let y = y as usize;
        fb[(y * 64 + x) * 4] = 255;
        fb[(y * 64 + x) * 4 + 1] = 255;
        fb[(y * 64 + x) * 4 + 2] = 255;
        fb[(y * 64 + x) * 4 + 3] = 255;
    }

    pub fn unset_pixel(&mut self, x: u8, y: u8) {
        let fb = self.0.frame_mut();
        let x = x as usize;
        let y = y as usize;
        fb[(y * 64 + x) * 4] = 0;
        fb[(y * 64 + x) * 4 + 1] = 0;
        fb[(y * 64 + x) * 4 + 2] = 0;
        fb[(y * 64 + x) * 4 + 3] = 0;
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        let fb = self.0.frame();
        let (x, y) = (x as usize, y as usize);
        if fb[(y * 64 + x) * 4] == 0
            && fb[(y * 64 + x) * 4 + 1] == 0
            && fb[(y * 64 + x) * 4 + 2] == 0
        {
            false
        } else {
            true
        }
    }

    pub fn clear_screen(&mut self) {
        self.0.frame_mut().fill(0);
    }

    pub fn flush(&mut self) {}
}
