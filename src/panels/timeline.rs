use egui::{include_image, pos2, vec2, Color32, Image, ImageButton, Pos2, Rect, Shape, Stroke, Ui};

use crate::Yanimator;

pub struct Timeline {
    zoom: f32,
    keyframe_image: Image<'static>
}

impl Timeline {
    pub fn init() -> Timeline {
        Timeline { 
            zoom: 10.0,
            keyframe_image: egui::Image::new(egui::include_image!("../../assets/keyframe.png"))
        }
    }
}

const KEYFRAME_SIZE: f32 = 30.0;

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {  
    let height = ui.available_height();

    ui.horizontal(|ui| {
        ui.add(ImageButton::new(include_image!("../../assets/frame_left.png")));
        ui.add(ImageButton::new(include_image!("../../assets/play.png")));
        ui.add(ImageButton::new(include_image!("../../assets/frame_right.png")));
    });

    ui.horizontal(|ui| {
        for i in 0..100 {
            if i % 10 != 0 { continue; }

            ui.painter().line_segment(
                [
                    pos2(i as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0, ui.cursor().min.y + KEYFRAME_SIZE),
                    pos2(i as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0, ui.cursor().min.y + height)
                ],
                Stroke::new(2.0, Color32::from_gray(60))
            );
            
            ui.put(egui::Rect::from_min_size(
                pos2(i as f32 * app.timeline.zoom, ui.cursor().min.y), vec2(KEYFRAME_SIZE, KEYFRAME_SIZE)
            ), |ui: &mut Ui| {
                ui.label(format!("{}", i))
            });
        }
        
        let mut pos: f32 = 0.0;
        
        for frame in &app.animations[app.animation_id].frames {
            ui.put(egui::Rect::from_min_size(
                pos2(pos * app.timeline.zoom, ui.cursor().min.y + height / 2.0 - KEYFRAME_SIZE / 2.0), vec2(KEYFRAME_SIZE, KEYFRAME_SIZE)
            ), |ui: &mut Ui| {
                ui.add(Image::new(include_image!("../../assets/keyframe.png")))
            }).on_hover_text(frame.cell.clone());
            pos += frame.duration as f32;
        }
    });


    ui.add_space(ui.available_height());
}