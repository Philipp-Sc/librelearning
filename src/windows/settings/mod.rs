#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsDisplay {
    pub auto_play_audio: bool,
    pub enable_sounds: bool,
    pub featch_new_card_at_threshold: bool,
    pub add_new_card_threshold: f32,
    pub spelling_correction_threshold: usize,
    pub ignore_punctuation_symbols: bool,
    pub review_mistakes: bool,

    #[serde(skip)]
    pub reset_app: bool,
}

impl Default for SettingsDisplay {
    fn default() -> Self {
        Self {
            auto_play_audio: false,
            enable_sounds: true,
            featch_new_card_at_threshold: true,
            add_new_card_threshold: 66.6,
            spelling_correction_threshold: 1,
            ignore_punctuation_symbols: true,
            review_mistakes: true,
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
                [available_rect.max.x - 20.0, available_rect.max.y - 60.0].into(),
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
                ui.checkbox(
                    &mut self.featch_new_card_at_threshold,
                    egui::RichText::new("Automaticaly Fetch New Cards").size(16.0),
                );

                ui.add(
                    egui::Slider::new(&mut self.add_new_card_threshold, 0.0..=100.0)
                        .text(egui::RichText::new("New Card Threshold").size(16.0)),
                );

                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Fetch New Card Now").size(16.0), //.color(egui::Color32::BLACK),
                        ), //.fill(egui::Color32::GREEN),
                    )
                    .clicked()
                {
                    /*
                    if self.settings_display.enable_sounds {
                        self.static_audio
                            .play_audio(&StaticSounds::MessageNewInstant).ok();
                    }*/
                    // TODO trigger new card to load. // and call next so the new card gets scheduled.
                }

                ui.separator();

                ui.add(
                    egui::Slider::new(&mut self.spelling_correction_threshold, 0..=10)
                        .text(egui::RichText::new("Permitted Spelling Mistakes").size(16.0)),
                );

                ui.checkbox(
                    &mut self.ignore_punctuation_symbols,
                    egui::RichText::new("Ignore Punctuation Symbols").size(16.0),
                );
                ui.checkbox(
                    &mut self.review_mistakes,
                    egui::RichText::new("Review Mistakes").size(16.0),
                );

                ui.separator();

                ui.checkbox(
                    &mut self.auto_play_audio,
                    egui::RichText::new("Auto Play Audio").size(16.0),
                );
                ui.checkbox(
                    &mut self.enable_sounds,
                    egui::RichText::new("Enable Sounds").size(16.0),
                );

                ui.separator();
                /*
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Connect Datasource")
                            .size(18.0)
                            //.color(egui::Color32::WHITE),
                    ))
                    .clicked()
                {}*/

                // take up the available space, but not the last 20 pixels.
                let mut available_space: egui::Vec2 = ui.available_size();
                available_space.y = available_space.y - 40.0;
                ui.allocate_space(available_space);

                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Reset App").size(16.0), //.color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    self.reset_app = true;
                }
            });
        });
    }
}
