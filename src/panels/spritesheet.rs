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

    ui.label("Spritesheet");
    ui.horizontal(|ui| {
        ui.label("Preview Palette ID");
        ui.add(egui::DragValue::new(&mut app.spritesheet_palette).speed(0.2).range(0..=app.textures.len() - 1));
    });
    

    egui::Grid::new("spritesheet_grid").spacing(vec2(-SPRITE_SIZE,0.0)).show(ui, |ui| {
        let mut i = 0;

        while i < app.textures[app.spritesheet_palette].len() {
            let mut texture = egui::Image::new(&app.textures[app.spritesheet_palette][i]).fit_to_exact_size(vec2(SPRITE_SIZE, SPRITE_SIZE));
            
            if oam.get_sprite_indexes_one_dimensional().iter().find(|&&x| x == i).is_some() {
                texture = texture.tint(Color32::LIGHT_GREEN);
            }

            let sprite = ui.add(texture);

            if (i + 1) % 32 == 0 {
                ui.end_row();
            }

            i += 1;
        }
    });

    ui.add_space(ui.available_height());
}