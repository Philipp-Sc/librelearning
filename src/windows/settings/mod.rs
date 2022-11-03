#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsDisplay {
    pub auto_play_audio: bool,
    pub enable_sounds: bool,
    pub add_new_card_threshold: f32,
    pub strict_input_comparison: bool,

    #[serde(skip)]
    pub reset_app: bool,
}

impl Default for SettingsDisplay {
    fn default() -> Self {
        Self {
            auto_play_audio: false,
            enable_sounds: true,
            add_new_card_threshold: 66.6,
            strict_input_comparison: false,
            reset_app: false,
        }
    }
}

impl super::Window for SettingsDisplay {
    fn name(&self) -> &'static str {
        "App Settings"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let available_rect = ctx.available_rect();

        egui::Window::new(self.name())
            .fixed_rect(egui::Rect::from_min_size(
                [available_rect.min.x + 5.0, available_rect.min.y + 40.0].into(),
                [available_rect.max.x - 20.0, available_rect.max.y - 90.0].into(),
            ))
            .open(open)
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for SettingsDisplay {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Connect/Edit Datasource")
                            .size(18.0)
                            .color(egui::Color32::WHITE),
                    ))
                    .clicked()
                {}
                ui.separator();

                ui.add(
                    egui::Slider::new(&mut self.add_new_card_threshold, 0.0..=100.0)
                        .text("Add New Card Threshold"),
                );

                ui.checkbox(&mut self.strict_input_comparison, "Strict Review");
                ui.checkbox(&mut self.auto_play_audio, "Auto Play Audio");
                ui.checkbox(&mut self.enable_sounds, "Enable Sounds");

                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Reset App")
                            .size(18.0)
                            .color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    self.reset_app = true;
                }
            });
        });
    }
}
