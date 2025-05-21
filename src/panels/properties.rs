use egui::Ui;

use crate::{anim_parser::{AnimationCel, OAMFlip, OAMShape, OAMSize}, Yanimator};

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    ui.heading("Properties");
    
    let cell  = match app.animation_cels.get_mut(&app.editing_cell) {
        Some(cell) => cell,
        None => return,
    };
    
    let oam = match cell.oams.get_mut(app.editing_oam) {
        Some(oam) => oam,
        None => return,
    };
    
    egui::Grid::new("animation_cells")
        .num_columns(2)
        .striped(true)
        .spacing([40.0, 4.0])
        .show(ui, |ui| {
        
            ui.label("X");
            ui.add(egui::DragValue::new(&mut oam.x).speed(0.2));
            ui.end_row();

            ui.label("Y");
            ui.add(egui::DragValue::new(&mut oam.y).speed(0.2));
            ui.end_row();
        
            ui.label("Tile ID");
            ui.add(egui::DragValue::new(&mut oam.tile).speed(0.2).range(0..=app.spritesheet.sprites.len()));
            ui.end_row();

            ui.label("Palette ID");
            ui.add(egui::DragValue::new(&mut oam.palette).speed(0.2).range(0..=app.palette.palettes.len() - 1));
            ui.end_row();
            
            ui.label("Shape");
            egui::ComboBox::from_id_salt("shape_dropdown")
                .selected_text(format!("{:?}", &mut oam.shape))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut oam.shape, OAMShape::Square, "Square");
                    ui.selectable_value(&mut oam.shape, OAMShape::Horizontal, "Horizontal");
                    ui.selectable_value(&mut oam.shape, OAMShape::Vertical, "Vertical");
                });
            ui.end_row();

            ui.label("Size");
            egui::ComboBox::from_id_salt("size_dropdown")
                .selected_text(format!("{:?}", &mut oam.size))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut oam.size, OAMSize::Size0, "Size0");
                    ui.selectable_value(&mut oam.size, OAMSize::Size1, "Size1");
                    ui.selectable_value(&mut oam.size, OAMSize::Size2, "Size2");
                    ui.selectable_value(&mut oam.size, OAMSize::Size3, "Size3");
                });
            ui.end_row();

            ui.label("Flip");
            egui::ComboBox::from_id_salt("flip_dropdown")
                .selected_text(format!("{:?}", &mut oam.flip))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut oam.flip, OAMFlip::None, "None");
                    ui.selectable_value(&mut oam.flip, OAMFlip::Horizontal, "Horizontal");
                    ui.selectable_value(&mut oam.flip, OAMFlip::Vertical, "Vertical");
                    ui.selectable_value(&mut oam.flip, OAMFlip::Both, "Both");
                });
            ui.end_row();
        });

    let rect = egui::Rect::from_min_size(
        ui.cursor().min,
        egui::vec2(ui.available_width().max(1.0), ui.available_height())
    );

    ui.allocate_rect(rect, egui::Sense::hover());
}