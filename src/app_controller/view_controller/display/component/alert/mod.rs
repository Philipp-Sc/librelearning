use crate::app_controller::view_controller::display::DisplayViewModel;
use crate::app_controller::view_controller::display::WindowViewModel;
use crate::app_controller::view_controller::view_model_controller::view_model::ControllerRequest;
use crate::app_controller::view_controller::view_model_controller::view_model::DisplayKind;
use crate::app_controller::view_controller::view_model_controller::view_model::{
    PropertieKey, PropertieValue,
};
use crate::app_controller::ViewModel;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AlertDisplay {}

impl Default for AlertDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl WindowViewModel for AlertDisplay {
    fn show(&mut self, ctx: &egui::Context, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            if let Some(PropertieValue::String(ref alert_text)) =
                inner.properties.get(&PropertieKey::Alert)
            {
            } else {
                return;
            }
        }
        let available_rect = ctx.available_rect();

        egui::Window::new("Alert")
            .fixed_rect(egui::Rect::from_min_size(
                [
                    available_rect.min.x,
                    available_rect.min.y + ((available_rect.max.y - available_rect.min.y) * (0.66)),
                ]
                .into(),
                [
                    available_rect.max.x,
                    ((available_rect.max.y - available_rect.min.y) * (0.34)),
                ]
                .into(),
            ))
            .frame(
                egui::containers::Frame::none()
                    .fill(egui::Color32::BLACK)
                    .inner_margin(egui::style::Margin {
                        left: 0.0,
                        right: 0.0,
                        top: 10.0,
                        bottom: 10.0,
                    }),
            )
            .anchor(egui::Align2::LEFT_BOTTOM, [0.0, 0.0])
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui, view_model);
            });
    }
}

impl DisplayViewModel for AlertDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, view_model: &ViewModel) {
        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            ui.label(
                egui::RichText::new(" Alert:")
                    .color(egui::Color32::RED)
                    .strong()
                    .monospace()
                    .size(15.0),
            );

            ui.allocate_space(egui::Vec2 { x: 0.0, y: 5.0 });
        });

        if let Ok(mut inner) = view_model.inner.lock() {
            if let Some(PropertieValue::String(ref alert_text)) =
                inner.properties.get_mut(&PropertieKey::Alert)
            {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.allocate_space(egui::Vec2 { x: 20.0, y: 0.0 });
                    ui.label(
                        egui::RichText::new(alert_text)
                            .color(egui::Color32::RED)
                            .size(20.0)
                            .monospace(),
                    );
                });
            }
        }

        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });

            let check = ui.add_sized(
                [ui.available_width() - 40.0, 30.0],
                egui::Button::new(
                    egui::RichText::new("OK")
                        .size(10.0)
                        .strong()
                        .monospace()
                        .color(egui::Color32::BLACK),
                )
                .fill(egui::Color32::RED),
            );
            if check.clicked() {
                if let Ok(mut inner) = view_model.inner.lock() {
                    inner
                        .controller_requests
                        .insert(ControllerRequest::HideAlert);
                }
            }
        });
    }
}
