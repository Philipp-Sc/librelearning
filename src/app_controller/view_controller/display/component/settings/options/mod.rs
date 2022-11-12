use crate::app_controller::view_controller::display::DisplayViewModel;
use crate::app_controller::view_controller::display::WindowViewModel;
use crate::app_controller::view_controller::view_model_controller::view_model::DisplayKind;
use crate::app_controller::view_controller::view_model_controller::view_model::{
    ControllerRequest, PropertieKey, PropertieValue,
};
use crate::app_controller::ViewModel;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OptionsSettingsDisplay {}

impl Default for OptionsSettingsDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl WindowViewModel for OptionsSettingsDisplay {
    fn show(&mut self, ctx: &egui::Context, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            if !(DisplayKind::OptionsSettingsDisplay == inner.display_kind) {
                return;
            }
        }
        let available_rect = ctx.available_rect();
        egui::Window::new("Options Settings")
            .fixed_rect(egui::Rect::from_min_size(
                [available_rect.min.x + 5.0, available_rect.min.y + 140.0].into(),
                [available_rect.max.x - 20.0, available_rect.max.y].into(),
            ))
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui, view_model);
            });
    }
}

impl DisplayViewModel for OptionsSettingsDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    if let Some(PropertieValue::Usize(ref mut spelling_correction_threshold)) =
                        inner
                            .properties
                            .get_mut(&PropertieKey::SpellingCorrectionThreshold)
                    {
                        ui.add(
                            egui::Slider::new(spelling_correction_threshold, 0..=10).text(
                                egui::RichText::new("Permitted spelling mistakes").size(16.0),
                            ),
                        );
                    }

                    if let Some(PropertieValue::Bool(ref mut ignore_sentence_punctuation_symbols)) =
                        inner
                            .properties
                            .get_mut(&PropertieKey::IgnoreSentencePunctuationSymbols)
                    {
                        ui.checkbox(
                            ignore_sentence_punctuation_symbols,
                            egui::RichText::new("Ignore sentence punctuation symbols").size(16.0),
                        );
                    }

                    if let Some(PropertieValue::Bool(ref mut match_ascii)) =
                        inner.properties.get_mut(&PropertieKey::MatchASCII)
                    {
                        ui.checkbox(match_ascii, egui::RichText::new("Match ASCII").size(16.0));
                    }

                    if let Some(PropertieValue::Bool(ref mut match_case)) =
                        inner.properties.get_mut(&PropertieKey::MatchCase)
                    {
                        ui.checkbox(
                            match_case,
                            egui::RichText::new("Match case (Lowercase/Uppercase)").size(16.0),
                        );
                    }
                    ui.separator();

                    if let Some(PropertieValue::Bool(ref mut auto_play_audio)) =
                        inner.properties.get_mut(&PropertieKey::AutoPlayAudio)
                    {
                        ui.checkbox(
                            auto_play_audio,
                            egui::RichText::new("Auto play audio").size(16.0),
                        );
                    }

                    if let Some(PropertieValue::Bool(ref mut enable_sound)) =
                        inner.properties.get_mut(&PropertieKey::EnableSounds)
                    {
                        ui.checkbox(
                            enable_sound,
                            egui::RichText::new("Enable sounds").size(16.0),
                        );
                    }
                },
            );
        }
    }
}
