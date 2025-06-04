use std::time::Instant;
use std::collections::HashMap;

use eframe::egui;
use egui::{Rect, TextureHandle};
use egui_extras::install_image_loaders;
use palette_parser::Palette;
use panels::{animation_cells::AnimationCellsPanel, timeline::Timeline};
use sprite_parser::Spritesheet;
use anim_parser::{Animation, AnimationCel};

mod palette_parser;
mod sprite_parser;
mod anim_parser;
mod export;
mod panels;
mod import;

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
    animation_cells_panel: AnimationCellsPanel,
    spritesheet_palette: usize
}

impl Yanimator {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {             
        let textures = Vec::new();
        let spritesheet = Spritesheet { sprites: Vec::new() };
        let palette = Palette { palettes: Vec::new() };
        let animation_cels = HashMap::new();
        let animations = Vec::new();

        Self {
            state: AppState::AnimationEditor,
            textures,
            animation_id: 0,
            spritesheet, 
            palette, 
            animation_cels,
            animations,
            last_frame_time: Instant::now(),
            frames: 0,
            viewport_rect: Rect::ZERO,
            timeline: Timeline::init(),
            animation_cells_panel: AnimationCellsPanel::init(),
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
        
        let animation = self.animations.get_mut(self.animation_id);
        
        if let Some(animation) = animation {
            if self.frames >= animation.get_total_frames() {
                self.frames = 0;
            }
            
            animation.current_frame = animation.get_anim_frame_from_frames(self.frames);
        }
        
        ctx.request_repaint();
        
        ctx.input(|i| {
            panels::timeline::input(i, self);
        });

        egui::TopBottomPanel::top("menu")
            .show(ctx, |ui| {
                panels::menu_bar::ui(ui, self);
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