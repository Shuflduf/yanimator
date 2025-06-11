use egui::{include_image, vec2, Button, Id, ImageButton, Modal, Ui};

use crate::{anim_parser::Animation, AppState, Yanimator};

pub struct Topbar {
    pub animation_creation_modal_open: bool,
    animation_deletion_modal_open: bool,
    deleting_anim: Option<String>,
    animation_name: String
}

impl Topbar {
    pub fn init() -> Self {
        Self {
            animation_creation_modal_open: false,
            animation_deletion_modal_open: false,
            deleting_anim: None,
            animation_name: String::new()
        }
    }
}

fn is_anim_name_invalid(app: &mut Yanimator) -> Option<String> {
    let anim_name = &app.topbar.animation_name;
    
    if anim_name.len() == 0 {return None}
    if anim_name.contains(" ") {return Some(String::from("Animation name must not contain spaces"))}
    if !anim_name.chars().nth(0).unwrap().is_alphabetic() {return Some(String::from("First character in animation name must be a letter"))}
    if app.animations.iter().any(|animation| &animation.name == anim_name) {return Some(String::from("Animation name has already been used"))}

    None
}

fn create_animation(app: &mut Yanimator) {
    app.topbar.animation_creation_modal_open = false;

    app.animations.push(Animation {
        frames: Vec::new(),
        name: app.topbar.animation_name.clone(),
        current_frame: 0,
        duration: 0
    })
}

fn remove_animation(app: &mut Yanimator) {
    if let Some(deleting_anim) = &app.topbar.deleting_anim {
        let anim_pos = app.animations.iter().position(|anim| &anim.name == deleting_anim);
        
        if let Some(anim_pos) = anim_pos {
            app.animations.remove(anim_pos);
        }
    }
}

pub fn ui_animation_editor(ui: &mut Ui, app: &mut Yanimator) {
    egui::ScrollArea::horizontal()
    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            let mut i = 0;
            
            for animation in &mut app.animations {
                let button = ui.button(&animation.name);

                if button.clicked() {
                    animation.current_frame = 0;
                    app.animation_id = i;
                    app.frames = 0;
                    app.timeline.keyframes.clear();
                }

                button.context_menu(|ui| {
                    if ui.add(Button::image_and_text(include_image!("../../assets/delete.png"), "Delete")).clicked() {
                        app.topbar.animation_deletion_modal_open = true;
                        app.topbar.deleting_anim = Some(animation.name.clone());
                        ui.close_menu();
                    }
                });
                
                i += 1;
            }

            if ui.add(Button::image_and_text(include_image!("../../assets/add.png"), "New Animation")).clicked() {
                app.topbar.animation_creation_modal_open = true;
                app.topbar.animation_name = String::new();
            }
        });
    });

    if app.topbar.animation_creation_modal_open {
        Modal::new(Id::new("animation_creation")).show(ui.ctx(), |ui| {
            ui.heading("Creating Animation");
            ui.separator();
            
            ui.label("Animation Name:");
            
            let field = ui.text_edit_singleline(&mut app.topbar.animation_name);
            
            match is_anim_name_invalid(app) {
                Some(message) => field.show_tooltip_text(message),
                None => {}
            }

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    app.topbar.animation_creation_modal_open = false;
                }

                if ui.button("Create").clicked() && app.topbar.animation_name.len() > 0 && is_anim_name_invalid(app).is_none() {
                    create_animation(app)
                }
            });
        });
    } else if app.topbar.animation_deletion_modal_open {
        Modal::new(Id::new("animation_deletion")).show(ui.ctx(), |ui| {
            if let Some(deleting_anim) = &app.topbar.deleting_anim {
                ui.heading("Confirm Deletion");
                let separator = ui.separator();

                ui.label(format!("Are you sure you want to delete {}?", deleting_anim));
                
                ui.allocate_ui_with_layout(vec2(separator.rect.width(), ui.available_height()), egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Delete").clicked() {
                        remove_animation(app);
                        app.topbar.animation_deletion_modal_open = false;
                    }
                    
                    if ui.button("Cancel").clicked() {
                        app.topbar.animation_deletion_modal_open = false;
                    }
                });
            }
        });
    }
}

pub fn ui_cell_editor(ui: &mut Ui, app: &mut Yanimator) {
    egui::ScrollArea::horizontal()
    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.add(ImageButton::new(include_image!("../../assets/back.png"))).clicked() {
                app.state = AppState::AnimationEditor;
                
                for (_, cell) in &mut app.animation_cels {
                    for oam in &mut cell.oams {
                        oam.selected = false;
                    }
                }
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

