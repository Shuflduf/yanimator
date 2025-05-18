use egui::Ui;

use crate::Yanimator;

pub fn ui(ui: &mut Ui, app: &mut Yanimator) {
    ui.horizontal(|ui| {
        for i in &app.animations {
            if ui.button(&i.name).clicked() {
                println!("{}", &i.name);
            }
        }
    });
}
