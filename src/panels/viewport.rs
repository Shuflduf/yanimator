use egui::{Rect, Scene, Ui};
use image::flat::View;

use crate::{AppState, Yanimator};

pub struct Viewport {
    scene_rect: Rect,
    pub selection_indicator_enabled: bool
}

impl Viewport {
    pub fn init() -> Self {
        Viewport {
            scene_rect: Rect::ZERO,
            selection_indicator_enabled: true
        }
    }
}

pub fn ui_animation_editor(ui: &mut Ui, app: &mut Yanimator) {
    Scene::default()
    .zoom_range(0.1..=4.0)
    .show(ui, &mut app.viewport.scene_rect, |ui| {
    
    //let animation = &app.animations[app.animation_id];
    
    let animation = app.animations.get(app.animation_id);
    if let Some(animation) = animation {
        if let Some(frame) = animation.frames.get(animation.current_frame) {
            if let Some(animation_cel) = app.animation_cels.get(&frame.cell) {
                animation_cel.draw(&app.textures, ui, false);
            }
        }
        
    }
    
    });
    
}

pub fn ui_cell_editor(ui: &mut Ui, app: &mut Yanimator) {
    Scene::default()
    .zoom_range(0.1..=4.0)
    .show(ui, &mut app.viewport.scene_rect, |ui| {

    //let animation = &app.animations[app.animation_id];

    if let Some(animation_cel) = app.animation_cels.get_mut(&app.editing_cell) {
        let mut i = 0;
        for oam in &mut animation_cel.oams {
            oam.selected = app.editing_oam == i;
            i += 1;
        }
        
        animation_cel.draw(&app.textures, ui, app.viewport.selection_indicator_enabled);
    }

    });
}

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    match &app.state {
        AppState::AnimationEditor => ui_animation_editor(ui, app),
        AppState::CellEditor => ui_cell_editor(ui, app),
    }
}