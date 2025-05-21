use egui::{include_image, vec2, ImageButton, MouseWheelUnit, Ui};

use crate::{AppState, Yanimator};

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    ui.heading("Animation Cells");
    
    egui::ScrollArea::vertical()
    .show(ui, |ui| {
        egui::Grid::new("animation_cells")
        .num_columns(1)
        .striped(true)
        .spacing([40.0, 4.0])
        .show(ui, |ui| {
            for (name, cell) in &app.animation_cels {
                ui.horizontal(|ui| {
                    ui.label(name);
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(ImageButton::new(include_image!("../../assets/edit.png"))).clicked() {
                            app.state = AppState::CellEditor;
                            app.editing_cell = name.clone();
                        }
                    });
                });
                
                ui.end_row();
            }
        });
        
    });

    let rect = egui::Rect::from_min_size(
        ui.cursor().min,
        egui::vec2(ui.available_width().max(1.0), ui.available_height())
    );
    ui.allocate_rect(rect, egui::Sense::hover());
}