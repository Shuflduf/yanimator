use egui::Ui;

use crate::Yanimator;

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    ui.heading("OAMs");
    
    let cell = match app.animation_cels.get(&app.editing_cell) {
        Some(cell) => cell,
        None => return,
    };


    egui::ScrollArea::vertical()
    .show(ui, |ui| {
        egui::Grid::new("oams")
        .num_columns(1)
        .striped(true)
        .spacing([40.0, 4.0])
        .show(ui, |ui| {
            let mut i = 0;
            
            for oam in &cell.oams {
                if ui.button(format!("OAM_{}", i)).clicked() {
                    app.editing_oam = i;
                }
                
                i += 1;
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