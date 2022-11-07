#[derive(serde::Deserialize, serde::Serialize)]
pub struct CardDisplay {
    play_audio_requested: bool,
}

impl CardDisplay {
    fn take_action(&mut self, card_display_data: &mut super::super::app::CardDisplayData) {
        if self.play_audio_requested {
            if card_display_data.play_audio() {
                self.play_audio_requested = false;
            }
        }
    }
}

impl Default for CardDisplay {
    fn default() -> Self {
        Self {
            play_audio_requested: false,
        }
    }
}

impl super::CardView for CardDisplay {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        card_display_data: &mut super::super::app::CardDisplayData,
    ) {
        self.take_action(card_display_data);

        ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });

        ui.add(
            egui::Label::new(egui::RichText::new(card_display_data.get_question()).heading())
                .wrap(true),
        );
        ui.separator();

        ui.allocate_space(egui::Vec2 { x: 0.0, y: 20.0 });

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let play_audio_text = if card_display_data.has_audio() {
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
                self.play_audio_requested = true;
            }

            ui.add(
                egui::Label::new(
                    egui::RichText::new(card_display_data.get_context())
                        .color(egui::Color32::WHITE)
                        .size(20.0),
                )
                .wrap(true),
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
        });
    }
}
