#[derive(serde::Deserialize, serde::Serialize)]
pub struct CardDisplay {}

impl Default for CardDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl super::CardView for CardDisplay {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        card_display_data: &mut super::super::app::CardDisplayData,
    ) {
        ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });
        ui.heading(egui::RichText::new(card_display_data.get_question()));
        ui.separator();

        ui.allocate_space(egui::Vec2 { x: 0.0, y: 20.0 });

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let play_audio_text = if card_display_data.has_audio_item() {
                "🔊"
            } else {
                "🔇"
            };

            if ui
                .add(
                    egui::Button::new(egui::RichText::new(play_audio_text).size(20.0)).frame(false),
                )
                .clicked()
            {
                card_display_data.play_audio_item();
            }

            ui.label(
                egui::RichText::new(card_display_data.get_context())
                    .color(egui::Color32::WHITE)
                    .size(20.0),
            );
        });

        ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });
        ui.vertical_centered_justified(|ui| {
            if let Some(img) = card_display_data.get_image() {
                img.show(ui); /*_size(ui, egui::Vec2 { x: 200.0, y: 200.0 }))*/
                ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });
            } else {
                ui.allocate_space(egui::Vec2 { x: 0.0, y: 20.0 });
            }
            // note this is likely not efficient
        });
    }
}
