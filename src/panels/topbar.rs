use egui::{vec2, Ui};

use crate::Yanimator;

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    egui::ScrollArea::horizontal()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
        ui.horizontal(|ui| {
            let mut i = 0;

            for animation in &app.animations {
                if ui.button(&animation.name).clicked() {
                    app.animation_id = i;
                }
                
                i += 1;
            }
        });
    });
    
}
