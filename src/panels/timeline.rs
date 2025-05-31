use egui::{include_image, pos2, vec2, Color32, Image, ImageButton, InputState, Key, PointerButton, Rect, Stroke, Ui};

use crate::Yanimator;


#[derive(Debug)]
pub struct Keyframe {
    input_rect: Rect,
    selected: bool,
    hovered: bool,
    id: usize
}

pub struct Timeline {
    pub rect: Rect,
    zoom: f32,
    scroll: f32,
    pub playing: bool,
    pub keyframes: Vec<Keyframe>,
    start_drag_x: f32
}

impl Timeline {
    pub fn init() -> Timeline {
        Timeline { 
            rect: Rect::ZERO,
            zoom: 10.0,
            scroll: 0.0,
            playing: false,
            keyframes: Vec::new(),
            start_drag_x: 0.0
        }
    }

    pub fn update_keyframe(&mut self, input_rect: Rect, frame_id: usize) {
        match self.keyframes.iter_mut().find(|k| k.id == frame_id) {
            Some(keyframe) => {
                keyframe.input_rect = input_rect;
            },
            None => {
                self.keyframes.push(Keyframe { input_rect: input_rect, selected: false, hovered: false, id: frame_id });
                
            }
        }
    }

    pub fn is_keyframe_selected(&mut self, frame_id: usize) -> bool {
        match self.keyframes.iter().find(|&k| k.id == frame_id) {
            Some(keyframe) => keyframe.selected,
            None => false
        }
    }
    
    pub fn is_keyframe_hovered(&mut self, frame_id: usize) -> bool {
        match self.keyframes.iter().find(|&k| k.id == frame_id) {
            Some(keyframe) => keyframe.hovered,
            None => false
        }
    }
}

const KEYFRAME_SIZE: f32 = 30.0;
const SCROLL_SPEED: f32 = 10.0;

fn draw_keyframe(ui: &mut Ui, height: f32, timeline: &mut Timeline, pos: f32, i: usize, is_end: bool) {
    let keyframe_rect = egui::Rect::from_min_size(
        pos2(
            pos * timeline.zoom + timeline.scroll, 
            ui.cursor().min.y + height / 2.0 - KEYFRAME_SIZE / 2.0
        ), 
        vec2(KEYFRAME_SIZE, KEYFRAME_SIZE)
    );

    let input_rect = egui::Rect::from_min_size(
        pos2(
            pos * timeline.zoom + timeline.scroll + KEYFRAME_SIZE / 4.0, 
            ui.cursor().min.y + height / 2.0 - KEYFRAME_SIZE / 2.0
        ), 
        vec2(KEYFRAME_SIZE / 2.0, KEYFRAME_SIZE)
    );

    timeline.update_keyframe(input_rect, i);
    
    ui.put(keyframe_rect, |ui: &mut Ui| {
        let mut source = if !is_end {
            Image::new(include_image!("../../assets/keyframe.png"))
        } else {
            Image::new(include_image!("../../assets/end_keyframe.png"))
        };

        if timeline.is_keyframe_hovered(i) {
            source = source.tint(Color32::GRAY);
        }

        if timeline.is_keyframe_selected(i) {
            source = source.tint(Color32::GREEN);
        }

        if timeline.is_keyframe_hovered(i) && timeline.is_keyframe_selected(i) {
            source = source.tint(Color32::LIGHT_GREEN);
        }

        ui.add(source)
    });
}

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

        let animation = app.animations.get_mut(app.animation_id);
        if let Some(animation) = animation {
            for frame in &animation.frames {
                draw_keyframe(ui, height, &mut app.timeline, pos, frame.id, false);
                pos += frame.duration as f32;
            }

            draw_keyframe(ui, height, &mut app.timeline, pos, animation.frames.len(), true);
        }
    });
    
    ui.add_space(ui.available_height());
}

pub fn input(input: &InputState, app: &mut Yanimator) {
    let mouse_pos = input.pointer.latest_pos().unwrap_or(pos2(0.0, 0.0));
    
    for event in input.events.clone() {
        match event {
            egui::Event::MouseWheel { unit: _, delta, modifiers: _ } => {
                if app.timeline.rect.contains(mouse_pos) {
                    if input.modifiers.ctrl {
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
    
    if input.pointer.button_down(PointerButton::Middle) {
        if app.timeline.rect.contains(mouse_pos) {
            app.frames = ((mouse_pos.x - app.timeline.scroll - KEYFRAME_SIZE / 2.0) / app.timeline.zoom) as usize;
        }
    }

    let mut deselect_others = None;
    
    for keyframe in &mut app.timeline.keyframes {
        keyframe.hovered = keyframe.input_rect.contains(mouse_pos);
        
        if input.pointer.button_down(PointerButton::Secondary) && keyframe.hovered {
            if !input.modifiers.shift {
                deselect_others = Some(keyframe.id);
            }

            keyframe.selected = true;
        }

        
    }
    
    if let Some(id) = deselect_others {   
        for keyframe in &mut app.timeline.keyframes {
            if keyframe.id != id {
                keyframe.selected = false;
            }
        }
    }

    let animation = app.animations.get_mut(app.animation_id);

    if input.pointer.button_pressed(PointerButton::Primary) {
        app.timeline.start_drag_x = mouse_pos.x;
    }
    
    if let Some(animation) = animation {
        for keyframe in &mut app.timeline.keyframes {
            if keyframe.selected && input.pointer.button_released(PointerButton::Primary) {
                animation.move_anim_frame(keyframe.id, ((mouse_pos.x - app.timeline.start_drag_x) / app.timeline.zoom) as isize);
            }
            
            if keyframe.selected && input.key_pressed(Key::Delete) {
                animation.remove_anim_frame(keyframe.id);
            }
        } 
    }
}