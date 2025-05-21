use egui::{include_image, ImageButton, Ui};

use crate::{AppState, Yanimator};

pub fn ui_animation_editor(ui: &mut Ui, app: &mut Yanimator) {
    egui::ScrollArea::horizontal()
    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            let mut i = 0;
            
            for animation in &mut app.animations {
                if ui.button(&animation.name).clicked() {
                    animation.current_frame = 0;
                    app.animation_id = i;
                    app.frames = 0;
                }
                
                i += 1;
            }
        });
    });
}

pub fn ui_cell_editor(ui: &mut Ui, app: &mut Yanimator) {
    egui::ScrollArea::horizontal()
    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.add(ImageButton::new(include_image!("../../assets/back.png"))).clicked() {
                app.state = AppState::AnimationEditor;
            };
            
            ui.label(format!("{}", app.editing_cell));
        });
    });
}

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    match app.state {
        AppState::AnimationEditor => ui_animation_editor(ui, app),
        AppState::CellEditor => ui_cell_editor(ui, app),
    }
}

