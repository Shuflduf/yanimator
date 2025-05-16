use egui::{pos2, vec2, Pos2, Rect, TextureHandle, Ui};

use crate::Yanimator;

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
    palette: usize,
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
        
        let palette = (word3 >> 0xc) as usize;
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
        
        let width: usize;
        let height: usize;
        
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
        
        let mut y_range: Vec<usize> = (0..height).collect();
        let mut x_range: Vec<usize> = (0..width).collect();
        
        match self.flip {
            OAMFlip::Horizontal => {
                x_range = (0..width).rev().collect();
            },
            OAMFlip::Vertical => {
                y_range = (0..height).rev().collect();
            },
            OAMFlip::Both => {
                x_range = (0..width).rev().collect();
                y_range = (0..height).rev().collect();
            },
            OAMFlip::None => {}
        }

        for y in y_range {
            let mut row: Vec<usize> = Vec::new();
            
            for &x in &x_range {
                row.push(self.tile as usize + x + y * 32);
            }

            sprite_indexes.push(row);
        }
        
        return sprite_indexes;
    }

    pub fn draw(&self, textures: &Vec<Vec<TextureHandle>>, offset: Pos2, size: f32, ui: &mut Ui) {
        let oam_sprites = self.get_sprite_indexes();
            
        let sprite_size = size;

        for y in 0..oam_sprites.len() {
            for x in 0..oam_sprites[y].len() {
                let rect = egui::Rect::from_min_size(
                    pos2(
                        (x as f32) * sprite_size + (self.x as f32) * sprite_size / 8.0 + offset.x, 
                        (y as f32) * sprite_size + (self.y as f32) * sprite_size / 8.0 + offset.y),
                    vec2(sprite_size, sprite_size)
                );
                
                ui.put(rect, |ui: &mut Ui| {
                    let mut texture = egui::Image::new(&textures[self.palette][oam_sprites[y][x]]);
                    
                    match self.flip { 
                        OAMFlip::Horizontal => {
                            texture = texture.uv(Rect::from_min_max(pos2(1.0, 0.0), pos2(0.0, 1.0)));
                        },
                        OAMFlip::Vertical => {
                            texture = texture.uv(Rect::from_min_max(pos2(0.0, 1.0), pos2(1.0, 0.0)));
                        },
                        OAMFlip::Both => {
                            texture = texture.uv(Rect::from_min_max(pos2(1.0, 1.0), pos2(1.0, 1.0)));
                        },
                        _ => {}
                    }
                    
                    ui.add(
                        texture.fit_to_exact_size(vec2(sprite_size, sprite_size))
                    )
                });

                ui.allocate_space(vec2(sprite_size, sprite_size));
            }
        }
    }
}

pub struct AnimationCel {
    oams: Vec<OAM>
}

fn parse_hex_string(string: &str) -> Option<u8> {
    match u8::from_str_radix(&string, 16) {
        Ok(value) => Some(value),
        Err(_) => None
    }
}

impl AnimationCel {
    pub fn from_c(c: &str) -> Option<AnimationCel> {
        let length_start = c.find("/* Len */ ")? + 10;
        let mut length_str: String = String::from("");
        let length: usize;

        for i in length_start..length_start + 3 {
            if c.chars().nth(i) != Some(',') {
                length_str.push(c.chars().nth(i).unwrap());
            } else {
                break;
            }
        }

        length = match length_str.parse() {
            Ok(value) => value,
            Err(_) => return None,
        };

        let mut oam_positions = Vec::new();
        let mut i = length_start + 2;

        while let Some(pos) = c[i..].find("*/ ") {
            oam_positions.push(i + pos);
            i += pos + 4;
        }

        let mut oams: Vec<OAM> = Vec::new();

        for pos in oam_positions.into_iter() {
            let mut bytes: Vec<u8> = Vec::new();
            
            let byte1 = parse_hex_string(&c[pos+5..pos+7])?;
            let byte2 = parse_hex_string(&c[pos+7..pos+9])?;
            let byte3 = parse_hex_string(&c[pos+13..pos+15])?;
            let byte4 = parse_hex_string(&c[pos+15..pos+17])?;
            let byte5 = parse_hex_string(&c[pos+21..pos+23])?;
            let byte6 = parse_hex_string(&c[pos+23..pos+25])?;
            
            bytes.push(byte1);
            bytes.push(byte2);
            bytes.push(byte3);
            bytes.push(byte4);
            bytes.push(byte5);
            bytes.push(byte6);

            let oam = OAM::new(&bytes);
            oams.push(oam);
        }

        Some(AnimationCel { oams })
    }

    pub fn draw(&self, textures: &Vec<Vec<TextureHandle>>, offset: Pos2, size: f32, ui: &mut Ui) {
        for oam in self.oams.iter().rev() {
            oam.draw(textures, offset, size, ui);
        }
    }
}