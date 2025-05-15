use eframe::egui;
use egui::{epaint, pos2, vec2, Color32, ColorImage, Rect, TextureHandle, Ui};
use palette_parser::Palette;
use sprite_parser::{Sprite, Spritesheet};
use anim_parser::OAM;

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
        
        let test_oam = OAM::new(&vec![0x00, 0xe8, 0x41, 0xf8, 0x20, 0x9c]);
        println!("{:?}", test_oam);


        let test_oam2 = OAM::new(&vec![0x40, 0xf8, 0x01, 0xf8, 0x21, 0x52]);
        println!("{:?}", test_oam2);

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
            
            
            let test_oam = OAM::new(&vec![0x00, 0xe8, 0x81, 0xf0, 0x20, 0x0c]);
            
            //let test_oam = OAM::new(&vec![0x00, 0xf0, 0x81, 0xf0, 0x40, 0x08]);
            let oam_sprites = test_oam.get_sprite_indexes();
            
            let sprite_size = 20.0;

            for y in 0..oam_sprites.len() {
                for x in 0..oam_sprites[y].len() {
                    let rect = Rect::from_min_size(
                        pos2(
                            x as f32 * sprite_size as f32 + 50.0, 
                            y as f32 * sprite_size + test_oam.y as f32 + 50.0),
                        vec2(sprite_size, sprite_size)
                    );
                    
                    ui.put(rect, |ui: &mut Ui| {
                        ui.add(egui::Image::new(
                            &self.textures[oam_sprites[y][x]]).fit_to_exact_size(vec2(sprite_size, sprite_size))
                        )
                    });

                    ui.allocate_space(vec2(sprite_size, sprite_size));
                }
            }
            
            ui.add_space(200.0);
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
            });
        });
    }
}