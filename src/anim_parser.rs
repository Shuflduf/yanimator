#[derive(Debug)]
pub enum OAMShape {
    Square,
    Horizontal,
    Vertical
}
#[derive(Debug)]
pub enum OAMSize {
    Size0,
    Size1,
    Size2,
    Size3
}
#[derive(Debug)]
pub enum OAMFlip {
    None,
    Horizontal,
    Vertical,
    Both
}

#[derive(Debug)]
pub struct OAM {
    shape: OAMShape,
    size: OAMSize,
    flip: OAMFlip,
    pub x: i16,
    pub y: i16,
    palette: u16,
    tile: u16
}

impl OAM {
    pub fn new(bytes: &[u8]) -> OAM {
        // 0xSYYY, 0xFXXX, 0xPTTT
        let word1 = ((bytes[0] as u16) << 8) | (bytes[1] as u16);
        let word2 = ((bytes[2] as u16) << 8) | (bytes[3] as u16);
        let word3 = ((bytes[4] as u16) << 8) | (bytes[5] as u16);
        
        // TODO: probably throw a warning if shape/size are invalid
        
        let shape = match word1 >> 0xc {
            0x0 => OAMShape::Square,
            0x4 => OAMShape::Horizontal,
            0x8 => OAMShape::Vertical,
            _ => OAMShape::Square
        };

        let mut y= (word1 & 0x0FFF) as i16;
        if y >= 0x80 {
            y -= 0x100;
        }
        
        let flip_size_nibble = word2 >> 0xc;
        let size = match flip_size_nibble & !0x3 {
            0x0 => OAMSize::Size0,
            0x4 => OAMSize::Size1,
            0x8 => OAMSize::Size2,
            0xc => OAMSize::Size3,
            _ => OAMSize::Size0
        };

        let flip = match flip_size_nibble - (flip_size_nibble & !0x3) {
            0x0 => OAMFlip::None,
            0x1 => OAMFlip::Horizontal,
            0x2 => OAMFlip::Vertical,
            0x3 => OAMFlip::Both,
            _ => OAMFlip::None
        };
        
        let mut x = (word2 & 0x0FFF) as i16;

        if x >= 0x80 {
            x -= 0x200;
        }
        
        let palette = word3 >> 0xc;
        let tile = word3 & 0x0FFF;
        
        OAM {
            shape,
            size,
            flip,
            x,
            y,
            palette,
            tile
        }
    }

    pub fn get_sprite_indexes(&self) -> Vec<Vec<usize>> {
        let mut sprite_indexes: Vec<Vec<usize>> = Vec::new();
        
        let mut width: usize;
        let mut height: usize;
        
        match self.shape {
            OAMShape::Square => match self.size {
                OAMSize::Size0 => { width = 1; height = 1; },
                OAMSize::Size1 => { width = 2; height = 2; },
                OAMSize::Size2 => { width = 4; height = 4; },
                OAMSize::Size3 => { width = 8; height = 8; },
            },
            OAMShape::Horizontal => match self.size {
                OAMSize::Size0 => { width = 2; height = 1; },
                OAMSize::Size1 => { width = 4; height = 1; },
                OAMSize::Size2 => { width = 4; height = 2; },
                OAMSize::Size3 => { width = 8; height = 4; },
            },
            OAMShape::Vertical => match self.size {
                OAMSize::Size0 => { width = 1; height = 2; },
                OAMSize::Size1 => { width = 1; height = 4; },
                OAMSize::Size2 => { width = 2; height = 4; },
                OAMSize::Size3 => { width = 4; height = 8; },
            }
        }
        
        for y in 0..height {
            let mut row: Vec<usize> = Vec::new();
            
            for x in 0..width {
                row.push(self.tile as usize + x + y * 32);
            }

            sprite_indexes.push(row);
        }
        
        return sprite_indexes;
    }
}