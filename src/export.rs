use std::{collections::HashMap, fs, io::Write};

use crate::anim_parser::{Animation, AnimationCel};

/*
    
.yan format:

YAN file signature (3 bytes)
4 bytes for offset for animations

For each animation cell:

    Animation Cell Name
    0x00 seperator byte
    Amount of OAMs (1 byte)

    For each OAM:
        (Each OAM is 8 bytes)
        [yea, i could probably squish it back together into 6 bytes like how its actually stored]
        [but is it really worth it?? these files tend to be a few kilobytes at most]
        shape
        size
        flip
        x
        y
        palette
        tile (2 bytes, upper byte then lower byte)

For each animation:

    Animation name
    0x00 seperator byte
    Length of animation frames in bytes (2 bytes) (...hopefully an animation will never be more than 65535 bytes long..!)
    
    For each animation frame:
        Frame name
        0x00 seperator byte
        Frame duration (1 byte)
*/

pub fn create_project_bin(animation_cells: &HashMap<String, AnimationCel>, animations: &Vec<Animation>) {
    let mut bytes: Vec<u8> = Vec::new();

    bytes.extend(String::from("YAN").as_bytes().to_vec());
    // We will set these bytes after putting in all the animation cells
    bytes.extend([0x00, 0x00, 0x00, 0x00]);

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

    let animation_start_index: u32 = bytes.len() as u32;

    bytes[3] = (animation_start_index >> 24) as u8;
    bytes[4] = ((animation_start_index & 0xFF0000) >> 16) as u8;
    bytes[5] = ((animation_start_index & 0xFF00) >> 8) as u8;
    bytes[6] = (animation_start_index & 0xFF) as u8;

    for animation in animations {
        // Animation name
        let name_bytes = animation.name.as_bytes().to_vec();
        bytes.extend(name_bytes);

        // Seperator byte
        bytes.push(0x00);
        
        // Allocate 2 bytes for animation length
        bytes.extend([0x00, 0x00]);
        let start_length = bytes.len();

        for frame in &animation.frames {
            bytes.extend(frame.cell.as_bytes().to_vec());
            bytes.push(0x00);
            bytes.push(frame.duration);
        }

        let end_length = bytes.len();
        let length = (end_length - start_length) as u16;

        bytes[start_length - 2] = (length >> 8) as u8;
        bytes[start_length - 1] = (length & 0xFF) as u8;
    }

    let export = fs::File::create("project.yan");
    
    if let Ok(mut file) = export {
        let _ = file.write_all(&bytes);
    }

    
}