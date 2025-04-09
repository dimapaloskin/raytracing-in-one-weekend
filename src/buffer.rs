use crate::color::{Color, vec3_to_color};

pub struct Buffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = vec![0; width * height];

        for pixel in buffer.iter_mut() {
            *pixel = 0x00_ffffff;
        }

        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: u32) -> &Self {
        self.buffer.fill(color);
        return self;
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: &Color) -> &Self {
        self.set(x, y, vec3_to_color(&color));
        return self;
    }

    fn set(&mut self, x: usize, y: usize, color: u32) -> &Self {
        let idx = y * self.width + x;
        self.buffer[idx] = color;
        return self;
    }

    pub fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) -> &Self {
        let r_x = if x < self.width { x } else { self.width - 1 };
        let r_y = if y < self.height { y } else { self.height - 1 };

        let r_width = if x + width < self.width {
            width
        } else {
            self.width - x
        };

        let r_height = if y + height < self.height {
            height
        } else {
            self.height - y
        };

        for i in r_x..r_x + r_width {
            for j in r_y..r_y + r_height {
                self.set(i, j, color);
            }
        }
        return self;
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
