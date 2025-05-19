use egui::{vec2, MouseWheelUnit, Ui};

use crate::Yanimator;

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
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

