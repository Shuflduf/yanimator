use egui::{include_image, pos2, vec2, Color32, Image, ImageButton, InputState, Key, PointerButton, Rect, Response, Scene, Stroke, Ui};

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
    start_drag_x: f32,
    dragging: bool
}

impl Timeline {
    pub fn init() -> Timeline {
        Timeline { 
            rect: Rect::ZERO,
            zoom: 10.0,
            scroll: 0.0,
            playing: false,
            keyframes: Vec::new(),
            start_drag_x: 0.0,
            dragging: true
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

fn draw_keyframe(ui: &mut Ui, height: f32, timeline: &mut Timeline, pos: f32, i: usize) -> Response {
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
    
    let response = ui.put(keyframe_rect, |ui: &mut Ui| {
        let mut source = Image::new(include_image!("../../assets/keyframe.png"));

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

    if timeline.dragging && timeline.is_keyframe_selected(i) {
        let mouse_pos = ui.ctx().input(|i| i.pointer.latest_pos().unwrap_or(pos2(0.0, 0.0)));
        let dragged_rect = keyframe_rect.translate(vec2(((mouse_pos.x - timeline.start_drag_x) / timeline.zoom).round() * timeline.zoom, 0.0));
        
        ui.put(dragged_rect, |ui: &mut Ui| {
            let mut source = Image::new(include_image!("../../assets/keyframe.png"));


            source = source.tint(Color32::from_rgba_unmultiplied(0, 255, 0, 128));
            
            ui.add(source)
        });
    }

    response
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

        let mut animation_end = 0;

        if let Some(animation) = app.animations.get(app.animation_id) {
            animation_end = animation.get_total_frames();
        }

        ui.label(format!("Frame {}/{}", app.frames, animation_end));


        if let Some(animation) = app.animations.get_mut(app.animation_id) {
            let minimum_duration = animation.get_minimum_duration();
            
            ui.label("Animation Duration: ");
            ui.add(egui::DragValue::new(&mut animation.duration).range(minimum_duration..=minimum_duration + 255));

            animation.update_duration();
        }
    });

    ui.separator();
    
    ui.horizontal(|ui| {
        let mut animation_end = 0;

        if let Some(animation) = app.animations.get(app.animation_id) {
            animation_end = animation.get_total_frames();
        }

        ui.painter().rect_filled(
            Rect::from_min_size(
                pos2(app.timeline.scroll + KEYFRAME_SIZE / 2.0, ui.cursor().min.y + KEYFRAME_SIZE), 
                vec2(animation_end as f32 * app.timeline.zoom, height)
            ), 
            0, Color32::from_gray(50));

        for i in 0..animation_end {
            if i % 10 != 0 { continue; }
            
            ui.painter().line_segment(
                [
                    pos2(i as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0 + app.timeline.scroll, ui.cursor().min.y + KEYFRAME_SIZE),
                    pos2(i as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0 + app.timeline.scroll, ui.cursor().min.y + height)
                ],
                Stroke::new(2.0, Color32::from_gray(70))
            );
            
            ui.put(egui::Rect::from_min_size(
                pos2(i as f32 * app.timeline.zoom + app.timeline.scroll, ui.cursor().min.y), vec2(KEYFRAME_SIZE, KEYFRAME_SIZE)
            ), |ui: &mut Ui| {
                ui.label(format!("{}", i))
            });
        }

        ui.painter().line_segment(
            [
                pos2(animation_end as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0 + app.timeline.scroll, ui.cursor().min.y + KEYFRAME_SIZE),
                pos2(animation_end as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0 + app.timeline.scroll, ui.cursor().min.y + height)
            ],
            Stroke::new(2.0, Color32::WHITE)
        );

        ui.put(egui::Rect::from_min_size(
            pos2(animation_end as f32 * app.timeline.zoom + app.timeline.scroll, ui.cursor().min.y), vec2(KEYFRAME_SIZE, KEYFRAME_SIZE)
        ), |ui: &mut Ui| {
            ui.label(format!("{}", animation_end))
        });

        ui.painter().line_segment(
            [
                pos2(app.timeline.scroll + app.frames as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0, ui.cursor().min.y + KEYFRAME_SIZE),
                pos2(app.timeline.scroll + app.frames as f32 * app.timeline.zoom + KEYFRAME_SIZE / 2.0, ui.cursor().min.y + height)
            ],
            Stroke::new(4.0, Color32::from_rgb(97, 134, 255))
        );
        
        let mut pos: f32 = 0.0;

        let animation = app.animations.get_mut(app.animation_id);
        if let Some(animation) = animation {
            for frame in &animation.frames {
                let keyframe = draw_keyframe(ui, height, &mut app.timeline, pos, frame.id);
                let cel = app.animation_cels.get(&frame.cell);

                if let Some(cel) = cel {
                    keyframe.on_hover_ui_at_pointer(|ui| {    
                        ui.label(&frame.cell);

                        ui.allocate_ui(vec2(100.0, 100.0), |ui| {
                            let mut rect = Rect::ZERO;                    
                            Scene::default()
                                .zoom_range(0.5..=0.5)
                                .show(ui, &mut rect, |ui| {
                                    cel.draw(&app.textures, ui);
                                });
                        });
                    });
                }
                
                pos += frame.duration as f32;
            }
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
    let mut any_hovered = false;

    for keyframe in &mut app.timeline.keyframes {
        keyframe.hovered = keyframe.input_rect.contains(mouse_pos);
        
        if keyframe.hovered {
            any_hovered = true;
        }

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

    if let Some(animation) = animation {
        for keyframe in &mut app.timeline.keyframes {
            if keyframe.hovered && keyframe.selected && input.pointer.button_pressed(PointerButton::Primary) {
                app.timeline.start_drag_x = mouse_pos.x;
                app.timeline.dragging = true;
            }

            if keyframe.selected && app.timeline.dragging && input.pointer.button_released(PointerButton::Primary) {
                animation.move_anim_frame(keyframe.id, ((mouse_pos.x - app.timeline.start_drag_x) / app.timeline.zoom).round() as isize);
            }
            
            if keyframe.selected && input.key_pressed(Key::Delete) {
                animation.remove_anim_frame(keyframe.id);
            }
        } 
    }

    if app.timeline.dragging && input.pointer.button_released(PointerButton::Primary) {
        app.timeline.dragging = false;
    }

    if !any_hovered && app.timeline.dragging == false && (input.pointer.button_down(PointerButton::Primary) || input.pointer.button_down(PointerButton::Secondary)) {
        for keyframe in &mut app.timeline.keyframes {
            keyframe.selected = false;
        }
    }

    // todo: there will probably be more text box modal things
    // there should be a common function to check for those
    // when more are added
    if app.animation_cells_panel.creation_modal_open {return;}

    if input.key_pressed(Key::Space) {
        app.timeline.playing = !app.timeline.playing;
    }

    if input.key_pressed(Key::ArrowLeft) {
        if app.frames > 0 { app.frames -= 1 };
    }

    if input.key_pressed(Key::ArrowRight) {
        app.frames += 1;
    }
}