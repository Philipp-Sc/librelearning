use crate::app_controller::view_controller::display::DisplayViewModel;
use crate::app_controller::view_controller::display::WindowViewModel;
use crate::app_controller::view_controller::view_model_controller::view_model::ControllerRequest;
use crate::app_controller::view_controller::view_model_controller::view_model::DisplayKind;
use crate::app_controller::view_controller::view_model_controller::view_model::{
    PropertieKey, PropertieValue, VolatilePropertieKey, VolatilePropertieValue,
};
use crate::app_controller::ViewModel;
use difference::Difference;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ReviewDisplay {}

impl Default for ReviewDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl WindowViewModel for ReviewDisplay {
    fn show(&mut self, ctx: &egui::Context, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            if inner
                .volatile_properties
                .get(&VolatilePropertieKey::Differences)
                .is_none()
            {
                return;
            }
        }

        let available_rect = ctx.available_rect();
        egui::Window::new("Review")
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

impl DisplayViewModel for ReviewDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            let mut score = false;
            if let Some(PropertieValue::Bool(ref val)) =
                inner.properties.get(&PropertieKey::ReviewScore)
            {
                score = *val;
            }
            if let Some(VolatilePropertieValue::Differences(ref differences)) = inner
                .volatile_properties
                .get(&VolatilePropertieKey::Differences)
            {
                let is_equal = differences.len() <= 1; // or <=1
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.label(
                        egui::RichText::new(if score {
                            if is_equal {
                                "  ✅ Excellent!"
                            } else {
                                "  ✅ You have a typo in your answer:"
                            }
                        } else {
                            "   ❌ Correct solution:"
                        }) // Nicely done. Meaning: // Excellent! // Nicely done. // Good job!
                        .color(if score {
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
                    for c in differences {
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
                            egui::RichText::new(if score { "CONTINUE" } else { "GOT IT" })
                                .size(10.0)
                                .strong()
                                .monospace()
                                .color(egui::Color32::BLACK),
                        )
                        .fill(if score {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        }),
                    );
                    if check.clicked() {
                        inner
                            .controller_requests
                            .insert(ControllerRequest::CloseReview);
                    }
                });
            }
        }
    }
}
