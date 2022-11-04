#[derive(serde::Deserialize, serde::Serialize)]
pub struct ReviewDisplay {
    close: bool,
    display_text: String,
    is_correct: bool,
}

impl Default for ReviewDisplay {
    fn default() -> Self {
        Self {
            close: false,
            display_text: "".to_owned(),
            is_correct: false,
        }
    }
}

impl super::ReviewWindow for ReviewDisplay {
    fn name(&self) -> &'static str {
        "Review"
    }

    fn show(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        label: &str,
        text: &str,
        score: &bool,
    ) {
        if self.close {
            *open = false;
            self.close = false;
        } else if *open {
            self.display_text = label.to_owned();
            self.is_correct = *score;

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

impl super::View for ReviewDisplay {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            ui.label(
                egui::RichText::new(if self.is_correct {
                    "  ✅ Excellent!"
                } else {
                    "   ❌ Correct solution:"
                }) // You have a typo in your answer: // Nicely done. Meaning: // Excellent! // Nicely done. // Good job!
                .color(if self.is_correct {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                })
                .strong()
                .monospace()
                .size(15.0),
            );

            ui.allocate_space(egui::Vec2 { x: 0.0, y: 5.0 });
        });
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(&self.display_text)
                    .color(if self.is_correct {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    })
                    .size(20.0)
                    .monospace(),
            );

            ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });

            let check = ui.add_sized(
                [ui.available_width() - 40.0, 30.0],
                egui::Button::new(
                    egui::RichText::new(if self.is_correct {
                        "CONTINUE"
                    } else {
                        "GOT IT"
                    })
                    .size(10.0)
                    .strong()
                    .monospace()
                    .color(egui::Color32::BLACK),
                )
                .fill(if self.is_correct {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                }),
            );
            if check.clicked() {
                self.close = true;
            }
        });
    }
}
