use std::fs;

#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct Palette {
    pub palettes: Vec<Vec<RGB>>
}

impl Palette {
    pub fn from_pal(file_path: &str) -> Result<Palette, std::io::Error> {
        // TODO: this will only read the first row of colors for now, implement other
        // rows of palette colors later
        
        

        let bytes = fs::read(file_path)?;
        let mut palettes = Vec::new();

        for pal in 0x0..0xF {
            let mut colors: Vec<RGB> = Vec::new();
            
            for i in 0x0..0x10 {
                let r = bytes[0x18 + i * 4 + pal * 0x40];
                let g = bytes[0x19 + i * 4 + pal * 0x40];
                let b = bytes[0x1a + i * 4 + pal * 0x40];
                
                colors.push(RGB {r, g, b});
            }

            palettes.push(colors);
        }

        

        Ok(Palette {
            palettes
        })
    }
}