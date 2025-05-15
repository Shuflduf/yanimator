use std::fs;

use eframe::egui;
use egui::{epaint, pos2, vec2, Color32, ColorImage, Pos2, Rect, TextureHandle, Ui};
use palette_parser::Palette;
use sprite_parser::{Sprite, Spritesheet};
use anim_parser::{AnimationCel, OAM};

use rayon::prelude::*;

mod palette_parser;
mod sprite_parser;
mod anim_parser;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Yanimator", native_options, Box::new(|cc| Ok(Box::new(Yanimator::new(cc)))))
}

struct Yanimator {
    textures: Vec<TextureHandle>,
    sprite_id: usize,
    palette: Palette,
    spritesheet: Spritesheet,
    offset: Pos2,
    zoom: f32,
    animation_cels: Vec<AnimationCel>
}

impl Yanimator {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load Palette
        let palette = Palette::from_pal("night_walk.pal").unwrap();

        for pal in palette.colors.iter() {
            println!("R: {}, G: {}, B: {}", pal.r, pal.g, pal.b);
        }


        // Load Spritesheet and create TextureHandles
        let spritesheet = Spritesheet::from_4bpp("night_walk_obj.4bpp").unwrap();
        let mut textures: Vec<TextureHandle> =  Vec::new();
        
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
                    let rgb = &palette.colors[palette_id as usize];
                    pixels.push(rgb.r);
                    pixels.push(rgb.g);
                    pixels.push(rgb.b);
                    pixels.push(255);
                }
                
            }
            
            textures.push(
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
        
        // Load AnimationCels

        //let animation_cels;
        let test_cels_file = fs::read_to_string("night_walk_anim_cels.c").unwrap();

        let mut cel_positions = Vec::new();
        let mut i = 0;

        while let Some(pos) = test_cels_file[i..].find("[] = {") {
            cel_positions.push(i + pos);
            i += pos + 7;
        }

        let animation_cels = cel_positions
            .par_iter()
            .filter_map(|&start| {
                let mut cel_str = String::new();

                for i in start..test_cels_file.len() {
                    if test_cels_file.chars().nth(i) != Some(';') {
                        cel_str.push(test_cels_file.chars().nth(i).unwrap());
                    } else {
                        break;
                    }
                }

                AnimationCel::from_c(&cel_str)
            })
            .collect();
        
        
        Self {
            textures,
            sprite_id: 0,
            spritesheet, 
            palette, 
            offset: pos2(0.0, 0.0),
            zoom: 20.0,
            animation_cels
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
            ui.heading(format!("{}", self.sprite_id));
            ui.add(egui::DragValue::new(&mut self.sprite_id).speed(0.1).range(0..=self.animation_cels.len()));
            /*let test_cel = AnimationCel::from_c(
                "
                
                AnimationCel night_walk_cel033[] = {
    /* Len */ 7,
    /* 000 */ 0x00f7, 0x01fc, 0x0160,
    /* 001 */ 0x00f7, 0x41f8, 0x008c,
    /* 002 */ 0x8008, 0x41fc, 0x0014,
    /* 003 */ 0x8028, 0x41fc, 0x0014,
    /* 004 */ 0x8048, 0x41fc, 0x0014,
    /* 005 */ 0x8068, 0x41fc, 0x0014,
    /* 006 */ 0x4000, 0x4004, 0x012f
};
                
                ");*/
            //let test_cel = ;
            //if let Some(cel) = test_cel {
                if let Some(animation_cel) = self.animation_cels.get(self.sprite_id) {
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