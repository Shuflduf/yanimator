use eframe::egui;
use egui::{epaint, pos2, vec2, Color32, ColorImage, Pos2, Rect, TextureHandle, Ui};
use palette_parser::Palette;
use sprite_parser::{Sprite, Spritesheet};
use anim_parser::{AnimationCel, OAM};

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
    offset: Pos2
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
            spritesheet, 
            palette, 
            offset: pos2(0.0, 0.0)
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
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("{}", self.sprite_id));
            
            let test_cel = AnimationCel::from_c(
                "
                
                AnimationCel night_walk_cel088[] = {
    /* Len */ 21,
    /* 000 */ 0x4000, 0x41bc, 0x010b,
    /* 001 */ 0x8009, 0x41bc, 0x0014,
    /* 002 */ 0x4000, 0x419c, 0x010b,
    /* 003 */ 0x8009, 0x419c, 0x0014,
    /* 004 */ 0x4000, 0x407c, 0x010b,
    /* 005 */ 0x8009, 0x407c, 0x0014,
    /* 006 */ 0x4000, 0x41dc, 0x010b,
    /* 007 */ 0x8009, 0x41dc, 0x0014,
    /* 008 */ 0x0000, 0x01fc, 0x010b,
    /* 009 */ 0x8009, 0x41fc, 0x0014,
    /* 010 */ 0x00f0, 0x001c, 0x010b,
    /* 011 */ 0x80f9, 0x401c, 0x0014,
    /* 012 */ 0x00e0, 0x003c, 0x010b,
    /* 013 */ 0x80e9, 0x403c, 0x0014,
    /* 014 */ 0x00d0, 0x005c, 0x010b,
    /* 015 */ 0x80d9, 0x405c, 0x0014,
    /* 016 */ 0x8019, 0x401c, 0x0014,
    /* 017 */ 0x8009, 0x403c, 0x0014,
    /* 018 */ 0x80f9, 0x401c, 0x0014,
    /* 019 */ 0x80f9, 0x405c, 0x0014,
    /* 020 */ 0x8019, 0x405c, 0x0014
};
                
                ");

            if let Some(cel) = test_cel {
                cel.draw(self, ui);
            }

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