use egui::{vec2, Color32, Ui};

use crate::Yanimator;

const SPRITE_SIZE: f32 = 20.0;

pub fn ui(ui: &mut Ui, app: &mut Yanimator) { 
    let cell  = match app.animation_cels.get_mut(&app.editing_cell) {
        Some(cell) => cell,
        None => return,
    };
    
    let oam = match cell.oams.get_mut(app.editing_oam) {
        Some(oam) => oam,
        None => return,
    };

    let texture_sheet = match app.textures.get(app.spritesheet_palette) {
        Some(texture_sheet) => texture_sheet,
        None => return
    };

    ui.label("Spritesheet");
    ui.horizontal(|ui| {
        ui.label("Preview Palette ID");
        let mut upper_range = app.palette.palettes.len();
            
        if upper_range > 0 {
            upper_range -= 1;
        }
        ui.add(egui::DragValue::new(&mut app.spritesheet_palette).speed(0.2).range(0..=upper_range));
    });
    

    egui::Grid::new("spritesheet_grid").spacing(vec2(-SPRITE_SIZE,0.0)).show(ui, |ui| {
        let mut i = 0;
        
        while i < texture_sheet.len() {
            let source = match texture_sheet.get(i) {
                Some(source) => source,
                None => continue
            };
            
            let mut texture = egui::Image::new(source).fit_to_exact_size(vec2(SPRITE_SIZE, SPRITE_SIZE));
            
            if oam.get_sprite_indexes_one_dimensional().iter().find(|&&x| x == i).is_some() {
                texture = texture.tint(Color32::LIGHT_GREEN);
            }

            let sprite = ui.add(texture);
            if sprite.clicked() {
                oam.tile = i;
            }

            if (i + 1) % 32 == 0 {
                ui.end_row();
            }

            i += 1;
        }
    });

    ui.add_space(ui.available_height());
}