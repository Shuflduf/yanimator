use std::cell;

use egui::{include_image, vec2, Id, ImageButton, Modal, Ui};

use crate::{anim_parser::AnimationCel, AppState, Yanimator};

pub struct AnimationCellsPanel {
    creation_modal_open: bool,
    cell_name: String
}

impl AnimationCellsPanel {
    pub fn init() -> Self {
        Self { 
            creation_modal_open: false,
            cell_name: String::from("")
        }
    }
}

fn is_cell_name_invalid(app: &mut Yanimator) -> Option<String> {
    let cell_name = &app.animation_cells_panel.cell_name;
    
    if cell_name.len() == 0 {return None}
    if cell_name.contains(" ") {return Some(String::from("Cell name must not contain spaces"))}
    if !cell_name.chars().nth(0).unwrap().is_alphabetic() {return Some(String::from("First letter in cell name must be a letter"))}
    if app.animation_cels.get(cell_name).is_some() {return Some(String::from("Cell name has already been used"))}

    None
}

fn create_animation_cell(app: &mut Yanimator) {
    app.animation_cells_panel.creation_modal_open = false;

    app.animation_cels.insert(app.animation_cells_panel.cell_name.clone(), AnimationCel {
        name: app.animation_cells_panel.cell_name.clone(),
        oams: Vec::new()
    });
}

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    ui.horizontal(|ui| {
        ui.heading("Animation Cells");
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add_sized(vec2(20.0, 20.0), ImageButton::new(include_image!("../../assets/add.png"))).clicked() {
                app.animation_cells_panel.creation_modal_open = true;
                app.animation_cells_panel.cell_name = String::from("");
            }
        });
    });
    
    ui.separator();

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
                        if ui.add_sized(vec2(20.0, 20.0), ImageButton::new(include_image!("../../assets/edit.png"))).clicked() {
                            app.state = AppState::CellEditor;
                            app.editing_cell = name.clone();
                        }

                        if ui.add_sized(vec2(20.0, 20.0), ImageButton::new(include_image!("../../assets/keyframe_add.png"))).clicked() {
                            let animation = app.animations.get_mut(app.animation_id);
                            
                            if let Some(animation) = animation {
                                animation.insert_anim_frame(name.clone(), app.frames as isize);
                            }
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

    if app.animation_cells_panel.creation_modal_open {
        Modal::new(Id::new("animation_cell_creation")).show(ui.ctx(), |ui| {
            ui.heading("Creating AnimationCel");
            ui.separator();
            
            ui.label("Cell Name:");
            let field = ui.text_edit_singleline(&mut app.animation_cells_panel.cell_name);
            
            match is_cell_name_invalid(app) {
                Some(message) => field.show_tooltip_text(message),
                None => {}
            }

            if ui.button("Create").clicked() && app.animation_cells_panel.cell_name.len() > 0 && is_cell_name_invalid(app).is_none() {
                create_animation_cell(app)
            }
        });
    }

    ui.allocate_rect(rect, egui::Sense::hover());
}