#[derive(serde::Deserialize, serde::Serialize)]
pub struct AlertDisplay {
    close: bool,
    display_text: String,
}

impl Default for AlertDisplay {
    fn default() -> Self {
        Self {
            close: false,
            display_text: "".to_string(),
        }
    }
}

impl super::AlertWindow for AlertDisplay {
    fn name(&self) -> &'static str {
        "Alert"
    }

    fn show(&mut self, ctx: &egui::Context, text: &mut Option<String>) {
        if self.close {
            *text = None;
            self.close = false;
        } else if let Some(text) = text {
            self.display_text = text.to_owned();

            let available_rect = ctx.available_rect();

            egui::Window::new(self.name())
                .fixed_rect(egui::Rect::from_min_size(
                    [
                        available_rect.min.x,
                        available_rect.min.y
                            + ((available_rect.max.y - available_rect.min.y) * (0.66)),
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
                    use super::View as _;
                    self.ui(ui);
                });
        }
    }
}

impl super::View for AlertDisplay {
    fn ui(&mut self, ui: &mut egui::Ui) {
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

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.allocate_space(egui::Vec2 { x: 20.0, y: 0.0 });
            ui.label(
                egui::RichText::new(&self.display_text)
                    .color(egui::Color32::RED)
                    .size(20.0)
                    .monospace(),
            );
        });

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
                self.close = true;
            }
        });
    }
}
