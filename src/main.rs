use std::{fs, time::Instant};
use std::collections::HashMap;

use eframe::egui;
use egui::{menu, ColorImage, Rect, TextureHandle};
use egui_extras::install_image_loaders;
use export::create_cells_bin;
use palette_parser::Palette;
use panels::timeline::Timeline;
use sprite_parser::Spritesheet;
use anim_parser::{Animation, AnimationCel};

use rayon::prelude::*;

mod palette_parser;
mod sprite_parser;
mod anim_parser;
mod export;
mod panels;

fn main() -> eframe::Result {
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Yanimator", native_options, Box::new(|cc| Ok(Box::new(Yanimator::new(cc)))))
}

#[derive(PartialEq)]

enum AppState {
    AnimationEditor,
    CellEditor
}

struct Yanimator {
    state: AppState,
    textures: Vec<Vec<TextureHandle>>,
    animation_id: usize,
    palette: Palette,
    spritesheet: Spritesheet,
    
    editing_cell: String,
    editing_oam: usize,
    animation_cels: HashMap<String, AnimationCel>,
    

    animations: Vec<Animation>,
    last_frame_time: Instant,
    frames: usize,
    viewport_rect: Rect,

    timeline: Timeline,
    spritesheet_palette: usize
}

/*const TEST_PALETTE: &str = "polyrhythm.pal";
const TEST_SPRITES: &str = "polyrhythm_obj.4bpp";
const TEST_ANIM_CELS: &str = "polyrhythm_anim_cels.c";
const TEST_ANIM: &str = "polyrhythm_anim.c";



const TEST_PALETTE: &str = "samurai_slice.pal";
const TEST_SPRITES: &str = "samurai_slice_obj.4bpp";
const TEST_ANIM_CELS: &str = "samurai_slice_anim_cels.c";
const TEST_ANIM: &str = "samurai_slice_anim.c";

const TEST_PALETTE: &str = "karate_man.pal";
const TEST_SPRITES: &str = "karate_man_obj.4bpp";
const TEST_ANIM_CELS: &str = "karate_man_anim_cells.inc.c";
const TEST_ANIM: &str = "karate_man_anim.c";*/

/*const TEST_PALETTE: &str = "tap_trial.pal";
const TEST_SPRITES: &str = "tap_trial_obj.4bpp";
const TEST_ANIM_CELS: &str = "tap_trial_anim_cels.c";
const TEST_ANIM: &str = "tap_trial_anim.c";



const TEST_PALETTE: &str = "night_walk.pal";
const TEST_SPRITES: &str = "night_walk_obj.4bpp";
const TEST_ANIM_CELS: &str = "night_walk_anim_cels.c";
const TEST_ANIM: &str = "night_walk_anim.c";*/


const TEST_PALETTE: &str = "clappy_trio.pal";
const TEST_SPRITES: &str = "clappy_trio_obj.4bpp";
const TEST_ANIM_CELS: &str = "clappy_trio_anim_cels.c";
const TEST_ANIM: &str = "clappy_trio_anim.c";

impl Yanimator {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let total_time = Instant::now();
        let mut step_time = Instant::now();

        // Load Palette
        let palette = Palette::from_pal(TEST_PALETTE).unwrap();

        println!("Loaded palettes: {:?}", step_time.elapsed());
        step_time = Instant::now();

        // Load Spritesheet and create TextureHandles
        let spritesheet = Spritesheet::from_4bpp(TEST_SPRITES).unwrap();

        println!("Loaded sprites: {:?}", step_time.elapsed());
        step_time = Instant::now();
        
        let mut textures: Vec<Vec<TextureHandle>> =  Vec::new();
        
        for pal in palette.palettes.iter() {
            let mut palette_textures = Vec::new();

            for i in 0..spritesheet.sprites.len() {
                let sprite = &spritesheet.sprites[i];
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
                    cc.egui_ctx.load_texture(
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

        println!("Created TextureHandles: {:?}", step_time.elapsed());
        step_time = Instant::now();
        
        // Load AnimationCels
        
        let test_cels_file = fs::read_to_string(TEST_ANIM_CELS).unwrap();
        
        let mut cel_positions = Vec::new();
        let mut i = 0;

        while let Some(pos) = test_cels_file[i..].find("AnimationCel ") {
            cel_positions.push(i + pos);
            i += pos + 7;
        }

        /*let animation_cels = cel_positions
            .par_iter()
            .filter_map(|&start| {
                let mut cel_str = String::new();
                let mut cel_name = String::new();

                for i in start + 13..test_cels_file.len() {
                    if test_cels_file.chars().nth(i) != Some('[') {
                        cel_name.push(test_cels_file.chars().nth(i).unwrap());
                    } else {
                        break;
                    }
                }

                for i in start + 13 + cel_name.len()..test_cels_file.len() {
                    if test_cels_file.chars().nth(i) != Some(';') {
                        cel_str.push(test_cels_file.chars().nth(i).unwrap());
                    } else {
                        break;
                    }
                }

                AnimationCel::from_c(&cel_str, &cel_name)
            })
            .map(|cel| (cel.name.clone(), cel))
            .collect();
        
        println!("Loaded AnimationCels: {:?}", step_time.elapsed());
        step_time = Instant::now();*/

        //create_cells_bin(&animation_cels);

        let mut cooler_animation_cels: HashMap<String, AnimationCel> = HashMap::new();

        let test_binary_cels = fs::read("cells.bin").unwrap();
        let mut current_start_index = 0;
        let mut current_end_index;
        let mut name_length = 0;
        let mut read_name = false;
        
        i = 0;


        while i < test_binary_cels.len() {
            println!("parsing byte {}", i);
            let byte = test_binary_cels[i];

            if byte == 0x00 && !read_name {
                read_name = true;
            }
            i += 1;
            name_length += 1;
            // We are now on the cell length byte
            if read_name {
                println!("parsed a cell, length of oams is {}", test_binary_cels[i]);
                current_end_index = current_start_index + name_length + test_binary_cels[i] as usize * 8 + 1;
                
                let cell = AnimationCel::from_bin(&test_binary_cels[current_start_index..current_end_index]);
                if let Some(cell) = cell {
                    cooler_animation_cels.insert(cell.name.clone(), cell);
                }
                
                
                read_name = false;
                

                name_length = 0;
                i = current_end_index;
                current_start_index = i;
               
            }
        }

        println!("Loaded AnimationCels: {:?}", step_time.elapsed());
        step_time = Instant::now();

        // Load Animations

        let test_anim_file = fs::read_to_string(TEST_ANIM).unwrap();

        let mut anim_positions = Vec::new();
        i = 0;

        while let Some(pos) = test_anim_file[i..].find("struct Animation ") {
            anim_positions.push(i + pos);
            i += pos + 17;
        }

        let animations = anim_positions
            .par_iter()
            .filter_map(|&start| {
                let anim_name_start = start + 17;
                let anim_name_end = test_anim_file[anim_name_start..]
                    .find('[')
                    .map(|pos| anim_name_start + pos)
                    .unwrap();

                let anim_name = &test_anim_file[anim_name_start..anim_name_end];

                let anim_str_start = anim_name_end + 1;
                let anim_str_end = test_anim_file[anim_str_start..]
                    .find(';')
                    .map(|pos| anim_str_start + pos)
                    .unwrap();

                let anim_str = &test_anim_file[anim_str_start..anim_str_end];
                
                Animation::from_c(&anim_str, &anim_name)
            })
            .collect();

        println!("Loaded Animations: {:?}", step_time.elapsed());
        println!("Total load time: {:?}", total_time.elapsed());
        Self {
            state: AppState::AnimationEditor,
            textures,
            animation_id: 0,
            spritesheet, 
            palette, 
            animation_cels: cooler_animation_cels,
            animations,
            last_frame_time: Instant::now(),
            frames: 0,
            viewport_rect: Rect::ZERO,
            timeline: Timeline::init(),
            editing_cell: String::from(""),
            editing_oam: 0,
            spritesheet_palette: 0
        }
    }
}

impl eframe::App for Yanimator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //ctx.set_debug_on_hover(true);
        install_image_loaders(ctx);
        
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time).as_secs_f32();
        
        if elapsed >= 1.0 / 60.0 {
            if self.timeline.playing {self.frames += 1;}
            self.last_frame_time = now;
        }
        
        let animation = &mut self.animations[self.animation_id];
        
        if self.frames == animation.get_total_frames() {
            self.frames = 0;
        }
        
        animation.current_frame = animation.get_anim_frame_from_frames(self.frames);
        
        ctx.request_repaint();
        
        ctx.input(|i| {
            panels::timeline::input(i, self);
        });

        egui::TopBottomPanel::top("menu")
            .show(ctx, |ui| {
                // Use the menu_bar function here
                menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New Project [Ctrl+N]").clicked() {
                            // Blahh
                        }
                        
                        if ui.button("Open Project [Ctrl+O]").clicked() {
                            // Blahh
                        }
                    });
                });
            });

        egui::TopBottomPanel::top("topbar")
            .show(ctx, |ui| {
                panels::topbar::ui(ui, self);
            });
        
        egui::TopBottomPanel::bottom("timeline")
            .resizable(true)
            .show(ctx, |ui|{
                match self.state {
                    AppState::AnimationEditor => panels::timeline::ui(ui, self),
                    AppState::CellEditor => panels::spritesheet::ui(ui, self),
                }
            });
        
        egui::SidePanel::left("animation_cells")
            .resizable(true)
            .show(ctx, |ui| {
                match self.state {
                    AppState::AnimationEditor => panels::animation_cells::ui(ui, self),
                    AppState::CellEditor => panels::oams::ui(ui, self),
                }
                
            });
        
        if self.state == AppState::CellEditor {
            egui::SidePanel::right("properties")
            .resizable(true)
            .show(ctx, |ui| {
                panels::properties::ui(ui, self)
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            panels::viewport::ui(ui, self)
        });
    }
}