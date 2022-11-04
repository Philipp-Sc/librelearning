use difference::{Changeset, Difference};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ReviewDisplay {
    close: bool,
    is_equal: bool,
    is_correct: bool,

    #[serde(skip)]
    diffs: Vec<Difference>,
}

impl Default for ReviewDisplay {
    fn default() -> Self {
        Self {
            close: false,
            is_equal: false,
            is_correct: false,
            diffs: Vec::new(),
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
            self.is_equal = label == text;
            self.is_correct = *score;

            let changeset = Changeset::new(label, text, "");
            self.diffs = changeset.diffs;

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
                    if self.is_equal {
                        "  ✅ Excellent!"
                    } else {
                        "  ✅ You have a typo in your answer:"
                    }
                } else {
                    "   ❌ Correct solution:"
                }) // Nicely done. Meaning: // Excellent! // Nicely done. // Good job!
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

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.allocate_space(egui::Vec2 { x: 20.0, y: 0.0 });
            for c in &self.diffs {
                match *c {
                    Difference::Same(ref z) => {
                        ui.label(
                            egui::RichText::new(z)
                                .color(egui::Color32::GREEN)
                                .size(20.0)
                                .monospace(),
                        );
                    }
                    Difference::Add(ref z) => {
                        ui.label(
                            egui::RichText::new(z)
                                .color(egui::Color32::WHITE)
                                .background_color(egui::Color32::RED)
                                .size(20.0)
                                .monospace(),
                        );
                    }
                    Difference::Rem(ref z) => {
                        ui.label(
                            egui::RichText::new(z)
                                .color(egui::Color32::WHITE)
                                .background_color(egui::Color32::RED)
                                .size(20.0)
                                .monospace(),
                        );
                    }
                }
            }
        });

        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            /*
            ui.label(
                egui::RichText::new(&self.display_text)
                    .color(if self.is_correct {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    })
                    .size(20.0)
                    .monospace(),
            );*/

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
