use std::fs;
pub struct Sprite {
    pub pixels: Vec<u8>
}

const LOWER_NIBBLE_MASK: u8 = 0b_00001111_u8;
const UPPER_NIBBLE_MASK: u8 = 0b_11110000_u8;

impl Sprite {
    pub fn from_4bpp(bytes: &[u8]) -> Sprite {
        let mut pixels: Vec<u8> = Vec::new();
        
        for i in 0..0x20 {
            let byte = bytes[i];
            let left_pixel = LOWER_NIBBLE_MASK & byte;
            let right_pixel = (UPPER_NIBBLE_MASK & byte) >> 4;

            pixels.push(left_pixel);
            pixels.push(right_pixel)
        }

        Sprite {
            pixels
        }
    }
}

pub struct Spritesheet {
    pub sprites: Vec<Sprite>
}

impl Spritesheet {
    pub fn from_4bpp(file_path: &str) -> Result<Spritesheet, std::io::Error> {
        let bytes = fs::read(file_path)?;
        let mut sprites: Vec<Sprite> = Vec::new();
        println!("{}", bytes.len() / 0x20);
        // todo: load more
        for i in 0..bytes.len() / 0x20 {
            let sprite = Sprite::from_4bpp(&bytes[(i * 0x20)..(i * 0x20) + 0x20]);
            sprites.push(sprite);
        }

        

        Ok(Spritesheet {
            sprites
        })
        
    }
}