use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(mut v: Vec3) -> RGB {
        v.x = if v.x > 255.0 {
            255.0
        } else if v.x < 0.0 {
            0.0
        } else {
            v.x
        };
        v.y = if v.y > 255.0 {
            255.0
        } else if v.y < 0.0 {
            0.0
        } else {
            v.y
        };
        v.z = if v.z > 255.0 {
            255.0
        } else if v.z < 0.0 {
            0.0
        } else {
            v.z
        };
        RGB {
            r: v.x as u8,
            g: v.y as u8,
            b: v.z as u8,
        }
    }
}

pub struct Image {
    w: u32,
    h: u32,
    buffer: Vec<u8>,
}

#[allow(dead_code)]
impl Image {
    pub fn new(w: u32, h: u32) -> Image {
        let size = 3 * w * h;
        let buffer = vec![0; size as usize];
        Image {
            w: w,
            h: h,
            buffer: buffer,
        }
    }

    pub fn buffer_size(&self) -> u32 {
        3 * self.w * self.h
    }

    pub fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.w * 3) + (x * 3);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let r = self.buffer[offset];
                let g = self.buffer[offset + 1];
                let b = self.buffer[offset + 2];
                Some(RGB { r: r, g: g, b: b })
            }
            None => None,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.buffer[offset] = color.r;
                self.buffer[offset + 1] = color.g;
                self.buffer[offset + 2] = color.b;
                true
            }
            None => false,
        }
    }

    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(path)?;
        let header = format!("P6 {} {} 255\n", self.w, self.h);

        file.write(header.as_bytes()).expect("Could not write File");
        file.write(&self.buffer).expect("Could not write File");

        Ok(())
    }
}
