use crate::app_controller::view_controller::display::DisplayViewModel;
use crate::app_controller::view_controller::display::WindowViewModel;
use crate::app_controller::view_controller::view_model_controller::view_model::DisplayKind;
use crate::app_controller::ViewModel;
pub mod api;
pub mod options;
pub mod save_load;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsDisplay {}

impl Default for SettingsDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl WindowViewModel for SettingsDisplay {
    fn show(&mut self, ctx: &egui::Context, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            if !(DisplayKind::APISettingsDisplay == inner.display_kind
                || DisplayKind::SaveLoadSettingsDisplay == inner.display_kind
                || DisplayKind::OptionsSettingsDisplay == inner.display_kind)
            {
                return;
            }
        }
        let available_rect = ctx.available_rect();
        egui::Window::new("Settings")
            .fixed_rect(egui::Rect::from_min_size(
                [available_rect.min.x + 5.0, available_rect.min.y + 40.0].into(),
                [available_rect.max.x - 20.0, available_rect.min.y].into(),
            ))
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui, view_model);
            });
    }
}

impl DisplayViewModel for SettingsDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::RIGHT).with_cross_justify(true),
                |ui| {
                    if ui
                        .add(egui::Button::new(
                            egui::RichText::new("API").size(16.0).color(
                                if inner.display_kind == DisplayKind::APISettingsDisplay {
                                    egui::Color32::WHITE
                                } else {
                                    egui::Color32::GRAY
                                },
                            ),
                        ))
                        .clicked()
                    {
                        inner.display_kind = DisplayKind::APISettingsDisplay;
                    }
                    if ui
                        .add(egui::Button::new(
                            egui::RichText::new("Options").size(16.0).color(
                                if inner.display_kind == DisplayKind::OptionsSettingsDisplay {
                                    egui::Color32::WHITE
                                } else {
                                    egui::Color32::GRAY
                                },
                            ),
                        ))
                        .clicked()
                    {
                        inner.display_kind = DisplayKind::OptionsSettingsDisplay;
                    }
                    if ui
                        .add(egui::Button::new(
                            egui::RichText::new("Save/Load").size(16.0).color(
                                if inner.display_kind == DisplayKind::SaveLoadSettingsDisplay {
                                    egui::Color32::WHITE
                                } else {
                                    egui::Color32::GRAY
                                },
                            ),
                        ))
                        .clicked()
                    {
                        inner.display_kind = DisplayKind::SaveLoadSettingsDisplay;
                    }
                },
            );
        }
    }
}
