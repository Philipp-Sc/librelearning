use crate::app_controller::view_controller::display::DisplayViewModel;
use crate::app_controller::view_controller::display::WindowViewModel;
use crate::app_controller::view_controller::view_model_controller::view_model::DisplayKind;
use crate::app_controller::view_controller::view_model_controller::view_model::{
    ControllerRequest, InnerViewModel, PropertieKey, PropertieValue, VolatilePropertieKey,
    VolatilePropertieValue,
};
use crate::app_controller::ViewModel;

use std::ops::Sub;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AppDisplay {}

impl Default for AppDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl AppDisplay {
    fn show_card(&mut self, ui: &mut egui::Ui, inner: &mut InnerViewModel) {
        ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });

        if let Some(PropertieValue::String(ref card_question)) =
            inner.properties.get(&PropertieKey::CardQuestion)
        {
            ui.add(egui::Label::new(egui::RichText::new(card_question).heading()).wrap(true));
        }
        ui.separator();

        ui.allocate_space(egui::Vec2 { x: 0.0, y: 20.0 });

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let mut play_audio_symbol = "🔇";

            if let Some(PropertieValue::Bool(ref card_has_audio)) =
                inner.properties.get(&PropertieKey::CardHasAudio)
            {
                if *card_has_audio {
                    play_audio_symbol = "🔊"
                }
            };

            if ui
                .add(
                    egui::Button::new(egui::RichText::new(play_audio_symbol).size(20.0))
                        .frame(false),
                )
                .clicked()
            {
                inner
                    .controller_requests
                    .insert(ControllerRequest::PlayCardAudio);
            }

            if let Some(PropertieValue::String(ref card_context)) =
                inner.properties.get(&PropertieKey::CardContext)
            {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(card_context)
                            .color(egui::Color32::WHITE)
                            .size(20.0),
                    )
                    .wrap(true),
                );
            }
        });

        ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });
        ui.vertical_centered_justified(|ui| {
            if let Some(VolatilePropertieValue::Image(ref image)) = inner
                .volatile_properties
                .get(&VolatilePropertieKey::CardImage)
            {
                if let Ok(retained_image) = image.lock() {
                    retained_image.show_size(ui, egui::Vec2 { x: 200.0, y: 200.0 });
                    ui.allocate_space(egui::Vec2 { x: 0.0, y: 10.0 });
                    return;
                }
            }
            ui.allocate_space(egui::Vec2 { x: 0.0, y: 20.0 });
        });
    }
}

impl WindowViewModel for AppDisplay {
    fn show(&mut self, ctx: &egui::Context, view_model: &ViewModel) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(mut inner) = view_model.inner.lock() {
                let progress = if let Some(PropertieValue::Float(val)) =
                    inner.properties.get(&PropertieKey::Progress)
                {
                    *val
                } else {
                    0.0
                };

                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add(
                        egui::widgets::ProgressBar::new(progress)
                            .desired_width(ui.available_width() - 36.0),
                    );

                    let response = ui.add(
                        egui::Button::new(egui::RichText::new("⚙").size(18.0))
                            .fill(egui::Color32::BLACK),
                    );

                    if response.clicked() {
                        if inner.display_kind == DisplayKind::AppDisplay {
                            inner.display_kind = DisplayKind::APISettingsDisplay;
                        } else {
                            inner.display_kind = DisplayKind::AppDisplay;
                        }
                    }
                });

                match inner.display_kind {
                    DisplayKind::AppDisplay => {
                        let mut show_fetch_new_card_button = false;

                        if let Some(PropertieValue::Bool(ref val)) =
                            inner.properties.get(&PropertieKey::ConnectToCustomServer)
                        {
                            show_fetch_new_card_button = *val;
                        }

                        let substract_from_height = if show_fetch_new_card_button {
                            90.0
                        } else {
                            40.0
                        };

                        let mut text_input_placeholder = "".to_string();

                        if let Some(PropertieValue::String(ref val)) =
                            inner.properties.get(&PropertieKey::UserTextInputHint)
                        {
                            text_input_placeholder.push_str(val);
                        }

                        self.show_card(ui, &mut inner);

                        if let Some(PropertieValue::String(ref mut text_input)) =
                            inner.properties.get_mut(&PropertieKey::UserTextInput)
                        {
                            let _text_input_response = ui.add_sized(
                                ui.available_size().sub(
                                    [
                                        0.0,
                                        substract_from_height, // this ensures the button show_fetch_new_card_button is not put behind the textinput.
                                    ]
                                    .into(),
                                ),
                                egui::TextEdit::multiline(text_input)
                                    .frame(false)
                                    .cursor_at_end(true)
                                    .hint_text(egui::RichText::new(text_input_placeholder)) // Type the Indonesian translation // Type what you hear
                                    .text_color(egui::Color32::WHITE)
                                    .font(egui::FontId::proportional(20.0))
                                    //.lock_focus(true)
                                    .desired_width(f32::INFINITY),
                            );
                        }

                        let mut available_space: egui::Vec2 = ui.available_size();
                        available_space.y = available_space.y - substract_from_height;
                        ui.allocate_space(available_space);

                        if show_fetch_new_card_button {
                            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                                let check = ui.add(
                                    egui::Button::new(egui::RichText::new("📥").size(40.0)).fill(
                                        egui::Color32::from_rgb(
                                            (255.0 - (255.0 * progress)) as u8,
                                            (255.0 * progress) as u8,
                                            (255.0 * 0.9) as u8,
                                        ),
                                    ),
                                );
                                if check.clicked() {
                                    inner
                                        .controller_requests
                                        .insert(ControllerRequest::FetchNewCard);
                                }
                            });
                        }

                        ui.with_layout(
                            egui::Layout::left_to_right(egui::Align::BOTTOM)
                                .with_main_justify(true),
                            |ui| {
                                let check =
                                    ui.add(egui::Button::new(egui::RichText::new("").size(30.0)));
                                if check.clicked() {
                                    inner
                                        .controller_requests
                                        .insert(ControllerRequest::CheckReview);
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        });
    }
}

impl DisplayViewModel for AppDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, view_model: &ViewModel) {}
}
