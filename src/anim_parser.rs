#[derive(Debug)]
pub enum OAMShape {
    Square,
    Horizontal,
    Vertical
}
#[derive(Debug)]
pub enum OAMSize {
    Size8,
    Size16,
    Size32,
    Size64
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
    x: u16,
    y: u16,
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

        let y = word1 & 0x0FFF;
        
        let flip_size_nibble = word2 >> 0xc;
        let size = match flip_size_nibble & !0x3 {
            0x0 => OAMSize::Size8,
            0x4 => OAMSize::Size16,
            0x8 => OAMSize::Size32,
            0xc => OAMSize::Size64,
            _ => OAMSize::Size8
        };

        let flip = match flip_size_nibble - (flip_size_nibble & !0x3) {
            0x0 => OAMFlip::None,
            0x1 => OAMFlip::Horizontal,
            0x2 => OAMFlip::Vertical,
            0x3 => OAMFlip::Both,
            _ => OAMFlip::None
        };
        
        let x = word2 & 0x0FFF;

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
}