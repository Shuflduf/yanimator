use std::{collections::HashMap, fs, path::PathBuf};

use egui::{menu, Button, ColorImage, Key, KeyboardShortcut, Modifiers, TextureHandle, Ui};
use rayon::prelude::*;

use crate::{anim_parser::{Animation, AnimationCel}, palette_parser::Palette, sprite_parser::Spritesheet, Yanimator};
use rfd::FileDialog;

const NEW_PROJECT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::N);
const OPEN_PROJECT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::O);
const SAVE_PROJECT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::S);

fn open_project(app: &mut Yanimator) {
    let file_path: PathBuf = match FileDialog::new()
    .add_filter("Yanimator project", &["yan"])
    .set_directory("/")
    .set_title("Select a Yanimator project")
    .pick_file() {
        Some(file) => file,
        None => return
    };

    let path_str = match file_path.to_str() {
        Some(path) => path,
        None => return
    };
    
    let project_bytes = fs::read(path_str).unwrap();

    let mut i;

    let mut animation_cels: HashMap<String, AnimationCel> = HashMap::new();
    
    
    let mut current_start_index = 7;
    let mut current_end_index;
    let mut name_length = 0;
    let mut read_name = false;
    
    i = 7;
    
    let animations_offset: u32 = ((project_bytes[3] as u32) << 24) | ((project_bytes[4] as u32) << 16) | ((project_bytes[5] as u32) << 8) | project_bytes[6] as u32;
    
    while i < animations_offset as usize {
        let byte = project_bytes[i];
        
        if byte == 0x00 && !read_name {
            read_name = true;
        }

        i += 1;
        name_length += 1;
        // We are now on the cell length byte
        if read_name {
            current_end_index = current_start_index + name_length + project_bytes[i] as usize * 8 + 1;
            
            let cell = AnimationCel::from_bin(&project_bytes[current_start_index..current_end_index]);
            if let Some(cell) = cell {
                animation_cels.insert(cell.name.clone(), cell);
            }
            
            read_name = false;
    
            name_length = 0;
            i = current_end_index;
            current_start_index = i;
        }
    }
    
    // Load Animations
    
    let mut animations = Vec::new();
    
    while i < project_bytes.len() {
        let byte = project_bytes[i];
        
        if byte == 0x00 && !read_name {
            read_name = true;
        }
        
        i += 1;
        name_length += 1;
    
        // We are now on the animation byte length

        if read_name {
            let upper_byte = project_bytes[i];
            i += 1;
            let lower_byte = project_bytes[i];
            let animation_length = (((upper_byte as u16) << 8) | lower_byte as u16) as usize;
            
            current_end_index = current_start_index + name_length + animation_length + 2;
            
            let animation = Animation::from_bin(&project_bytes[current_start_index..current_end_index]);
            
            if let Some(animation) = animation {
                animations.push(animation);
            }
            
            read_name = false;
            
            name_length = 0;
            i = current_end_index;
            current_start_index = i;
        }
    }
    
    app.animations = animations;
    app.animation_cels = animation_cels;
}

fn save_project(app: &mut Yanimator) {
    let file_path: PathBuf = match FileDialog::new()
    .add_filter("Yanimator project", &["yan"])
    .set_directory("/")
    .set_title("Select save location")
    .save_file() {
        Some(file) => file,
        None => return
    };

    let path_str = match file_path.to_str() {
        Some(path) => path,
        None => return
    };
    
    crate::export::create_project_bin(path_str, &app.animation_cels, &app.animations);
}

fn load_palette(ui: &mut Ui, app: &mut Yanimator) {
    let file_path: PathBuf = match FileDialog::new()
    .add_filter("Palette", &["pal"])
    .set_directory("/")
    .set_title("Select a palette")
    .pick_file() {
        Some(file) => file,
        None => return
    };
    
    let path_str = match file_path.to_str() {
        Some(path) => path,
        None => return
    };

    app.palette = Palette::from_pal(path_str).unwrap();
    load_texture_handles(ui, app);
}

fn load_spritesheet(ui: &mut Ui, app: &mut Yanimator) {
    let file_path: PathBuf = match FileDialog::new()
    .add_filter("4bpp spritesheet", &["4bpp"])
    .set_directory("/")
    .set_title("Select a spritesheet")
    .pick_file() {
        Some(file) => file,
        None => return
    };
    
    let path_str = match file_path.to_str() {
        Some(path) => path,
        None => return
    };

    app.spritesheet = Spritesheet::from_4bpp(path_str).unwrap();
    load_texture_handles(ui, app);
}

fn load_texture_handles(ui: &mut Ui, app: &mut Yanimator) {
    let mut textures: Vec<Vec<TextureHandle>> = Vec::new();
    
    if app.palette.palettes.len() == 0 {return;}
    if app.spritesheet.sprites.len() == 0 {return;}
    
    for pal in app.palette.palettes.iter() {
        let mut palette_textures = Vec::new();

        for i in 0..app.spritesheet.sprites.len() {
            let sprite = &app.spritesheet.sprites[i];
            let mut pixels: Vec<u8> = Vec::new();
        
            for i in 0..0x40 {
                let palette_id = sprite.pixels[i];
                
                if palette_id == 0 {
                    pixels.push(0);
                    pixels.push(0);
                    pixels.push(0);
                    pixels.push(0);
                } else {
                    let rgb = &pal[palette_id as usize];
                    pixels.push(rgb.r);
                    pixels.push(rgb.g);
                    pixels.push(rgb.b);
                    pixels.push(255);
                }
            }
            
            palette_textures.push(
                ui.ctx().load_texture(
                i.to_string(),
                ColorImage::from_rgba_unmultiplied([8, 8], &pixels), 
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    wrap_mode: egui::TextureWrapMode::Repeat,
                    mipmap_mode: None,
                })
            )
        }

        textures.push(palette_textures);
    }

    app.textures = textures;
}

fn load_animation_cels(app: &mut Yanimator) {
    let file_path: PathBuf = match FileDialog::new()
    .add_filter("C", &["c"])
    .set_directory("/")
    .set_title("Select Animation Cels")
    .pick_file() {
        Some(file) => file,
        None => return
    };
    
    let path_str = match file_path.to_str() {
        Some(path) => path,
        None => return
    };

    let mut i = 0;
    let cels_file = fs::read_to_string(path_str).unwrap();

    let mut cel_positions = Vec::new();


    while let Some(pos) = cels_file[i..].find("AnimationCel ") {
        cel_positions.push(i + pos + 13);
        i += pos + 7;
    }

    let animation_cels = cel_positions
        .par_iter()
        .filter_map(|&start| {
            let sliced_cel = &cels_file[start..];
            let cel_name_end = sliced_cel.find('[')?;
            let cel_name = &sliced_cel[..cel_name_end];

            let cel_str_start = cel_name_end + 1;
            let cel_str_end = sliced_cel[cel_str_start..].find(';')?;
            let cel_str = &sliced_cel[cel_str_start..cel_str_start + cel_str_end];

            AnimationCel::from_c(cel_str, cel_name)
        })
        .map(|cel| (cel.name.clone(), cel))
        .collect();

    app.animation_cels = animation_cels;
}

fn load_animations(app: &mut Yanimator) {
    let file_path: PathBuf = match FileDialog::new()
    .add_filter("C", &["c"])
    .set_directory("/")
    .set_title("Select Animations")
    .pick_file() {
        Some(file) => file,
        None => return
    };
    
    let path_str = match file_path.to_str() {
        Some(path) => path,
        None => return
    };

    let anim_file = fs::read_to_string(path_str).unwrap();

    let mut anim_positions = Vec::new();
    let mut i = 0;

    while let Some(pos) = anim_file[i..].find("struct Animation ") {
        anim_positions.push(i + pos + 17);
        i += pos + 17;
    }
    
    let animations = anim_positions
        .par_iter()
        .filter_map(|&start| {
            let sliced_anim = &anim_file[start..];
            let anim_name_end = sliced_anim.find('[')?;
            let anim_name = &sliced_anim[..anim_name_end];

            let anim_str_start = anim_name_end + 1;
            let anim_str_end = sliced_anim[anim_str_start..].find(';')?;
            let anim_str = &sliced_anim[anim_str_start..anim_str_start + anim_str_end];
            
            Animation::from_c(&anim_str, &anim_name)
        })
        .collect();

    app.animations = animations;
}

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.add(Button::new("New Project").shortcut_text(ui.ctx().format_shortcut(&NEW_PROJECT))).clicked() {
                // Blahh
            }
            
            if ui.add(Button::new("Open Project").shortcut_text(ui.ctx().format_shortcut(&OPEN_PROJECT))).clicked() {
                open_project(app);
            }
            
            if ui.add(Button::new("Save Project").shortcut_text(ui.ctx().format_shortcut(&SAVE_PROJECT))).clicked() {
                save_project(app);
            }

            ui.separator();

            if ui.button("Load Spritesheet (.4bpp)").clicked() {
                load_spritesheet(ui, app);
            }

            if ui.button("Load Palette (.pal)").clicked() {
                load_palette(ui, app);
            }
            
            if ui.button("Load Animation Cells (.c)").clicked() {
                load_animation_cels(app);
            }

            if ui.button("Load Animations (.c)").clicked() {
                load_animations(app);
            }
        });
    });
}