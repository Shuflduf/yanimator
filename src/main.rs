use eframe::egui;
use egui::{epaint, vec2, Color32, ColorImage, TextureHandle};
use palette_parser::Palette;
use sprite_parser::{Sprite, Spritesheet};

mod palette_parser;
mod sprite_parser;
mod anim_parser;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Yanimator", native_options, Box::new(|cc| Ok(Box::new(Yanimator::new(cc)))))
}

struct Yanimator {
    textures: Vec<TextureHandle>,
    sprite_id: u16,
    palette: Palette,
    spritesheet: Spritesheet
}

impl Yanimator {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let palette = Palette::from_pal("night_walk.pal").unwrap();

        for pal in palette.colors.iter() {
            println!("R: {}, G: {}, B: {}", pal.r, pal.g, pal.b);
        }

        let spritesheet = Spritesheet::from_4bpp("night_walk_obj.4bpp").unwrap();
        let mut textures: Vec<TextureHandle> =  Vec::new();
        
        for i in 0..spritesheet.sprites.len() {
            let sprite = &spritesheet.sprites[i];
            let mut pixels: Vec<u8> = Vec::new();
        
            for i in 0..0x40 {
                let palette_id = sprite.pixels[i];
                let rgb = &palette.colors[palette_id as usize];
                pixels.push(rgb.r);
                pixels.push(rgb.g);
                pixels.push(rgb.b);
            }
            
            textures.push(
                cc.egui_ctx.load_texture(
                i.to_string(),
                ColorImage::from_rgb([8, 8], &pixels), 
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    wrap_mode: egui::TextureWrapMode::Repeat,
                    mipmap_mode: None,
                })
            )
        }

        Self {
            textures,
            sprite_id: 0,
            spritesheet, palette
        }
    }
}

impl eframe::App for Yanimator {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_debug_on_hover(true);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            
            egui::Grid::new("spritesheet_grid").spacing(vec2(-10.0,0.0)).show(ui, |ui| {
                for i in 0..self.textures.len() {
                    ui.add(egui::Image::new(
                        &self.textures[i]).fit_to_exact_size(vec2(30.0, 30.0))
                    );

                    if (i + 1) % 16 == 0 {
                        ui.end_row();
                    }
                }
            });
            
            //ui.add(egui::Image::new(
            //    &self.textures[self.sprite_id as usize]).fit_to_exact_size(vec2(200.0, 200.0))
            //);
            //ui.add(egui::DragValue::new(&mut self.sprite_id).speed(0.1).range(0..=self.spritesheet.sprites.len() - 1));
        });
    }
}