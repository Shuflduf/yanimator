use std::{collections::HashMap, fs, io::Write};

use crate::anim_parser::AnimationCel;

pub fn create_cells_bin(animation_cells: &HashMap<String, AnimationCel>) {
    /*
    
    cells.bin format:

    animation_name
    0x00 seperator byte
    amount of OAMs (1 byte)
    [for each oam, 8 bytes]
    [yea, i could probably squish it back together into 6 bytes like how its actually stored]
    [but is it really worth it?? these files tend to be a few kilobytes at most]
    shape
    size
    flip
    x
    y
    palette
    tile (2 bytes, upper byte then lower byte)
    
    */


    let mut bytes: Vec<u8> = Vec::new();

    for (name, cell) in animation_cells {
        // Cell name (saved as these may be editable in the future)
        let name_bytes = name.as_bytes().to_vec();
        bytes.extend(name_bytes);

        // Seperator byte
        bytes.push(0x00);

        // Cell length
        bytes.push(cell.oams.len() as u8);

        for oam in &cell.oams {
            bytes.push(oam.shape as u8);
            bytes.push(oam.size as u8);
            bytes.push(oam.flip as u8);
            bytes.push(oam.x as u8);
            bytes.push(oam.y as u8);
            bytes.push(oam.palette as u8);
            bytes.push((oam.tile >> 8) as u8);
            bytes.push((oam.tile & 0xFF) as u8);
        }
    }

    let export = fs::File::create("cells.bin");
    
    if let Ok(mut file) = export {
        let _ = file.write_all(&bytes);
    }
}