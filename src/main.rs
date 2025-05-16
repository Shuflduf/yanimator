use std::fs;
use std::collections::HashMap;

use eframe::egui;
use egui::{epaint, pos2, vec2, Color32, ColorImage, Pos2, Rect, TextureHandle, Ui};
use palette_parser::Palette;
use sprite_parser::{Sprite, Spritesheet};
use anim_parser::{Animation, AnimationCel, OAM};

use rayon::prelude::*;

mod palette_parser;
mod sprite_parser;
mod anim_parser;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Yanimator", native_options, Box::new(|cc| Ok(Box::new(Yanimator::new(cc)))))
}

struct Yanimator {
    textures: Vec<Vec<TextureHandle>>,
    animation_id: usize,
    animation_cel_id: usize,
    palette: Palette,
    spritesheet: Spritesheet,
    offset: Pos2,
    zoom: f32,
    animation_cels: HashMap<String, AnimationCel>,
    animations: Vec<Animation>
}

/*const TEST_PALETTE: &str = "polyrhythm.pal";
const TEST_SPRITES: &str = "polyrhythm_obj.4bpp";
const TEST_ANIM_CELS: &str = "polyrhythm_anim_cels.c";
const TEST_ANIM: &str = "polyrhythm_anim.c";


const TEST_PALETTE: &str = "night_walk.pal";
const TEST_SPRITES: &str = "night_walk_obj.4bpp";
const TEST_ANIM_CELS: &str = "night_walk_anim_cels.c";
const TEST_ANIM: &str = "night_walk_anim.c";

const TEST_PALETTE: &str = "samurai_slice.pal";
const TEST_SPRITES: &str = "samurai_slice_obj.4bpp";
const TEST_ANIM_CELS: &str = "samurai_slice_anim_cels.c";
const TEST_ANIM: &str = "samurai_slice_anim.c";*/

const TEST_PALETTE: &str = "tap_trial.pal";
const TEST_SPRITES: &str = "tap_trial_obj.4bpp";
const TEST_ANIM_CELS: &str = "tap_trial_anim_cels.c";
const TEST_ANIM: &str = "tap_trial_anim.c";

impl Yanimator {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load Palette
        let palette = Palette::from_pal(TEST_PALETTE).unwrap();

        // Load Spritesheet and create TextureHandles
        let spritesheet = Spritesheet::from_4bpp(TEST_SPRITES).unwrap();
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

        //println!("{:?}", palette.palettes);
        
        // Load AnimationCels
        
        let test_cels_file = fs::read_to_string(TEST_ANIM_CELS).unwrap();
        
        let mut cel_positions = Vec::new();
        let mut i = 0;

        while let Some(pos) = test_cels_file[i..].find("AnimationCel ") {
            cel_positions.push(i + pos);
            i += pos + 7;
        }

        let animation_cels = cel_positions
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
                let mut anim_str = String::new();
                let mut anim_name = String::new();

                for i in start + 17..test_anim_file.len() {
                    if test_anim_file.chars().nth(i) != Some('[') {
                        anim_name.push(test_anim_file.chars().nth(i).unwrap());
                    } else {
                        break;
                    }
                }

                for i in start + 17 + anim_name.len()..test_anim_file.len() {
                    if test_anim_file.chars().nth(i) != Some(';') {
                        anim_str.push(test_anim_file.chars().nth(i).unwrap());
                    } else {
                        break;
                    }
                }
                Animation::from_c(&anim_str, &anim_name)
            })
            .collect();

        Self {
            textures,
            animation_id: 0,
            animation_cel_id: 0,
            spritesheet, 
            palette, 
            offset: pos2(0.0, 0.0),
            zoom: 20.0,
            animation_cels,
            animations
        }
    }
}

impl eframe::App for Yanimator {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       //ctx.set_debug_on_hover(true);

        let events = ctx.input(|i| i.events.clone());
        
        for event in events {
            match event {
                egui::Event::MouseMoved(pos) => {
                    ctx.input(|i| {
                        if i.pointer.button_down(egui::PointerButton::Secondary) {
                            self.offset += pos;
                        }
                    })
                }
                egui::Event::MouseWheel { unit, delta, modifiers } => {
                    self.zoom += delta.y;

                    if self.zoom <= 1.0 {
                        self.zoom = 1.0;
                    }
                }
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.add(egui::DragValue::new(&mut self.animation_id).speed(0.1).range(0..=self.animations.len() - 1));

            let animation = &self.animations[self.animation_id];
            ui.heading(format!("{}", animation.name));
            ui.add(egui::DragValue::new(&mut self.animation_cel_id).speed(0.1).range(0..=animation.frames.len() - 1));

            if let Some(animation_cel) = self.animation_cels.get(&animation.frames[self.animation_cel_id].cell) {
                ui.heading(format!("{}", animation_cel.name));
                animation_cel.draw(&self.textures, self.offset, self.zoom, ui);
            }
            //}

            

            //let test_oam = OAM::new(&vec![0x00, 0xe8, 0x41, 0xf8, 0x20, 0xd4]);
            //let test_oam2 = OAM::new(&vec![0x40, 0xf8, 0x01, 0xf8, 0x21, 0x48]);
            
            //test_oam2.draw(self, ui);
            //test_oam.draw(self, ui);

            //println!("{:?}, {:?}", test_oam, test_oam2);
            
            
            /*ui.add_space(100.0);
            ui.heading(format!("{}", self.sprite_id));
            egui::Grid::new("spritesheet_grid").spacing(vec2(-20.0,0.0)).show(ui, |ui| {
                let mut i = 0;
                //let mut alternate = true;
                while i < self.textures.len() {
                    let sprite = ui.add(egui::Image::new(
                        &self.textures[i]).fit_to_exact_size(vec2(20.0, 20.0))
                    );

                    if sprite.hovered() {
                        self.sprite_id = i;
                    }

                    if (i + 1) % 32 == 0 {
                        ui.end_row();
                    }

                    i += 1;
                }
            });*/
        });
    }
}