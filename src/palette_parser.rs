use std::fs;

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct Palette {
    pub colors: Vec<RGB>
}

impl Palette {
    pub fn from_pal(file_path: &str) -> Result<Palette, std::io::Error> {
        // TODO: this will only read the first row of colors for now, implement other
        // rows of palette colors later

        let mut colors: Vec<RGB> = Vec::new();

        let bytes = fs::read(file_path)?;
        
        for i in 0x0..0xF {
            let r = bytes[0x18 + i * 4];
            let g = bytes[0x19 + i * 4];
            let b = bytes[0x1a + i * 4];

            colors.push(RGB {r, g, b});
        }

        Ok(Palette {
            colors
        })
    }
}