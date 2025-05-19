use egui::{include_image, pos2, vec2, Color32, Image, ImageButton, InputState, Modifiers, MouseWheelUnit, PointerButton, Pos2, Rect, Shape, Stroke, Ui, Vec2};

use crate::Yanimator;

pub struct Timeline {
    pub rect: Rect,
    zoom: f32,
    scroll: f32,
    pub playing: bool
}

impl Timeline {
    pub fn init() -> Timeline {
        Timeline { 
            rect: Rect::ZERO,
            zoom: 10.0,
            scroll: 0.0,
            playing: false
        }
    }
}

const KEYFRAME_SIZE: f32 = 30.0;
const SCROLL_SPEED: f32 = 10.0;
pub fn ui(ui: &mut Ui, app: &mut Yanimator) {  
    app.timeline.rect = ui.max_rect();

    let height = ui.available_height();

    ui.horizontal(|ui| {
        if ui.add(ImageButton::new(include_image!("../../assets/frame_left.png"))).clicked() {
            if app.frames > 0 { app.frames -= 1 };
        };
        if ui.add(ImageButton::new(
            if app.timeline.playing {
                include_image!("../../assets/pause.png")
            } else {
                include_image!("../../assets/play.png")
            }
        )).clicked() {
            app.timeline.playing = !app.timeline.playing;
        };
        if ui.add(ImageButton::new(include_image!("../../assets/frame_right.png"))).clicked() {
            app.frames += 1;
        };
    });

    ui.horizontal(|ui| {
        ui.painter().line_segment(
            [
                pos2(app.timeline.scroll + app.frames as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0, ui.cursor().min.y + KEYFRAME_SIZE),
                pos2(app.timeline.scroll + app.frames as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0, ui.cursor().min.y + height)
            ],
            Stroke::new(4.0, Color32::from_rgb(97, 134, 255))
        );

        for i in 0..100 {
            if i % 10 != 0 { continue; }
            
            

            ui.painter().line_segment(
                [
                    pos2(i as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0 + app.timeline.scroll, ui.cursor().min.y + KEYFRAME_SIZE),
                    pos2(i as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0 + app.timeline.scroll, ui.cursor().min.y + height)
                ],
                Stroke::new(2.0, Color32::from_gray(60))
            );
            
            ui.put(egui::Rect::from_min_size(
                pos2(i as f32 * app.timeline.zoom + app.timeline.scroll, ui.cursor().min.y), vec2(KEYFRAME_SIZE, KEYFRAME_SIZE)
            ), |ui: &mut Ui| {
                ui.label(format!("{}", i))
            });
        }
        
        let mut pos: f32 = 0.0;
        
        for frame in &app.animations[app.animation_id].frames {
            ui.put(egui::Rect::from_min_size(
                pos2(pos * app.timeline.zoom + app.timeline.scroll, ui.cursor().min.y + height / 2.0 - KEYFRAME_SIZE / 2.0), vec2(KEYFRAME_SIZE, KEYFRAME_SIZE)
            ), |ui: &mut Ui| {
                ui.add(Image::new(include_image!("../../assets/keyframe.png")))
            }).on_hover_text(frame.cell.clone());
            pos += frame.duration as f32;
        }
    });


    ui.add_space(ui.available_height());
}

pub fn input(i: &InputState, app: &mut Yanimator) {
    let mouse_pos = i.pointer.hover_pos().unwrap_or(pos2(0.0, 0.0));
    for event in i.events.clone() {
        match event {
            egui::Event::MouseWheel { unit, delta, modifiers: _ } => {
                if app.timeline.rect.contains(mouse_pos) {
                    if i.modifiers.ctrl {
                        app.timeline.zoom += delta.y;

                        if app.timeline.zoom < 3.0 {
                            app.timeline.zoom = 3.0;
                        }
                    } else {
                        app.timeline.scroll += delta.y * SCROLL_SPEED;
                    }
                }
            },
            _ => {}
        }
    }
    
    if i.pointer.button_down(PointerButton::Secondary) {
        if app.timeline.rect.contains(mouse_pos) {
            app.frames = ((mouse_pos.x - app.timeline.scroll - KEYFRAME_SIZE / 2.0) / app.timeline.zoom) as usize;
        }
    }
}