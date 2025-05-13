use eframe::egui;
use egui::{epaint, vec2, Color32, ColorImage, TextureHandle};
use palette_parser::Palette;

mod palette_parser;

fn main() -> eframe::Result {
    let palette = Palette::from_pal("night_walk.pal").unwrap();

    for pal in palette.colors.iter() {
        println!("R: {}, G: {}, B: {}", pal.r, pal.g, pal.b);
    }

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Yanimator", native_options, Box::new(|cc| Ok(Box::new(Yanimator::new(cc)))))
}

struct Yanimator {
    test: TextureHandle
}

impl Yanimator {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut pixels: Vec<u8> = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                pixels.push(x * 15);
                pixels.push(y * 15);
                pixels.push(x * 15);
            }
        }

        Self {
            test: cc.egui_ctx.load_texture(
                "test",
                ColorImage::from_rgb([8, 8], &pixels), 
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    wrap_mode: egui::TextureWrapMode::Repeat,
                    mipmap_mode: None,
                })
        }
    }
}

impl eframe::App for Yanimator {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.add(egui::Image::new(
                &self.test).fit_to_exact_size(vec2(200.0, 200.0))
            );
        });
    }
}