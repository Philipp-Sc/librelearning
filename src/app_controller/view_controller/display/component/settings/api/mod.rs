use crate::app_controller::view_controller::display::DisplayViewModel;
use crate::app_controller::view_controller::display::WindowViewModel;
use crate::app_controller::view_controller::view_model_controller::view_model::DisplayKind;
use crate::app_controller::view_controller::view_model_controller::view_model::{
    ControllerRequest, PropertieKey, PropertieValue,
};
use crate::app_controller::ViewModel;

use crate::app_controller::model_controller::data_model::download::DownloadState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct APISettingsDisplay {}

impl Default for APISettingsDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl WindowViewModel for APISettingsDisplay {
    fn show(&mut self, ctx: &egui::Context, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            if !(DisplayKind::APISettingsDisplay == inner.display_kind) {
                return;
            }
        }
        let available_rect = ctx.available_rect();
        egui::Window::new("API Settings")
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

impl DisplayViewModel for APISettingsDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            let show_fetch_new_card_button =
                if let Some(PropertieValue::Bool(ref show_fetch_new_card_button)) =
                    inner.properties.get(&PropertieKey::ConnectToCustomServer)
                {
                    *show_fetch_new_card_button
                } else {
                    false
                };

            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    if let Some(PropertieValue::Bool(ref mut show_fetch_new_card_button)) = inner
                        .properties
                        .get_mut(&PropertieKey::ConnectToCustomServer)
                    {
                        ui.checkbox(
                            show_fetch_new_card_button,
                            egui::RichText::new("Connect to custom file server (📥)").size(16.0),
                        );
                    }

                    if show_fetch_new_card_button {
                        ui.separator();

                        let mut responses = Vec::new();

                        if let Some(PropertieValue::String(ref mut custom_server_endpoint)) = inner
                            .properties
                            .get_mut(&PropertieKey::CustomServerEndpoint)
                        {
                            let response = ui.add(
                                egui::TextEdit::singleline(custom_server_endpoint)
                                    .hint_text(egui::RichText::new("Enter your HTTP endpoint")),
                            );
                            responses.push(response); 
                        }

                        if let Some(PropertieValue::String(ref mut custom_server_username)) = inner
                            .properties
                            .get_mut(&PropertieKey::CustomServerUsername)
                        {
                            let response = ui.add(
                                egui::TextEdit::singleline(custom_server_username).hint_text(
                                    egui::RichText::new("Enter your user name (optional)"),
                                ),
                            );
                            responses.push(response); 
                        }

                        if let Some(PropertieValue::String(ref mut custom_server_password)) = inner
                            .properties
                            .get_mut(&PropertieKey::CustomServerPassword)
                        {
                            let response = ui.add(
                                egui::TextEdit::singleline(custom_server_password)
                                    .password(true)
                                    .hint_text(egui::RichText::new(
                                        "Enter your password (optional)",
                                    )),
                            );
                            responses.push(response);
                        }
                        for response in responses {
                            if response.changed() {
                                inner
                                    .controller_requests
                                    .insert(ControllerRequest::UpdateRequestConfig);
                                break;
                            }
                        }
                    }
                },
            );

            if show_fetch_new_card_button {
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                    |ui| {
                        if let Some(PropertieValue::DownloadState(ref state)) = inner
                            .properties
                            .get(&PropertieKey::CustomServerConnectionStatus)
                        {
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("Test connection")
                                            .size(16.0)
                                            .color(egui::Color32::BLACK),
                                    )
                                    .fill(match state {
                                        DownloadState::Done => egui::Color32::GREEN,
                                        DownloadState::ParseError => egui::Color32::GREEN,
                                        DownloadState::InProgress => egui::Color32::YELLOW,
                                        DownloadState::Failed(..) => egui::Color32::RED,
                                        DownloadState::Null => egui::Color32::RED,
                                        DownloadState::None => egui::Color32::GRAY,
                                    }),
                                )
                                .clicked()
                            {
                                inner
                                    .controller_requests
                                    .insert(ControllerRequest::TestCustomServerConnection(true));
                            }

                            ui.separator();
                        }
                    },
                );

                ui.with_layout(
                    egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                    |ui| {
                        ui.checkbox(
                            &mut false,
                            egui::RichText::new("Auto sync (upload) your local data").size(16.0),
                        );

                        ui.separator();

                        if let Some(PropertieValue::Bool(ref mut featch_new_card_at_threshold)) =
                            inner
                                .properties
                                .get_mut(&PropertieKey::FetchNextCardAtThreshold)
                        {
                            ui.checkbox(
                                featch_new_card_at_threshold,
                                egui::RichText::new("Auto fetch next card").size(16.0),
                            );
                        }

                        if let Some(PropertieValue::Float(ref mut add_new_card_threshold)) =
                            inner.properties.get_mut(&PropertieKey::AddNewCardThreshold)
                        {
                            ui.add(
                                egui::Slider::new(add_new_card_threshold, 0.0..=100.0)
                                    .text(egui::RichText::new("at progress pct.").size(16.0)),
                            );
                        }
                    },
                );
            }

            let mut connect_to_ai_server = false;

            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    ui.separator();

                    if let Some(PropertieValue::Bool(ref mut enable_gpt3_card_generation)) = inner
                        .properties
                        .get_mut(&PropertieKey::EnableGPT3CardGeneration)
                    {
                        connect_to_ai_server = *enable_gpt3_card_generation;
                        ui.checkbox(
                            enable_gpt3_card_generation,
                            egui::RichText::new("Enable GPT-3 card generation (☀)").size(16.0),
                        );
                    }

                    if let Some(PropertieValue::Bool(ref mut fetch_dalle_generated_images)) = inner
                        .properties
                        .get_mut(&PropertieKey::FetchDalleGeneratedImages)
                    {
                        connect_to_ai_server =
                            connect_to_ai_server || *fetch_dalle_generated_images;
                        ui.checkbox(
                            fetch_dalle_generated_images,
                            egui::RichText::new("Show DALL-E generated images (🌟)").size(16.0),
                        );
                    }

                    if connect_to_ai_server {
                        ui.separator();


                        let mut responses = Vec::new();

                        if let Some(PropertieValue::String(ref mut ai_server_endpoint)) =
                            inner.properties.get_mut(&PropertieKey::AIServerEndpoint)
                        {
                            let response = ui.add(
                                egui::TextEdit::singleline(ai_server_endpoint)
                                    .hint_text(egui::RichText::new("Enter AI HTTP endpoint")),
                            );

                            responses.push(response);
                        }

                        if let Some(PropertieValue::String(ref mut ai_server_username)) =
                            inner.properties.get_mut(&PropertieKey::AIServerUsername)
                        {
                            let response = ui.add(
                                egui::TextEdit::singleline(ai_server_username)
                                    .hint_text(egui::RichText::new("Enter your user name")),
                            );
                            responses.push(response);
                        }

                        if let Some(PropertieValue::String(ref mut ai_server_password)) =
                            inner.properties.get_mut(&PropertieKey::AIServerPassword)
                        {
                            let response = ui.add(
                                egui::TextEdit::singleline(ai_server_password)
                                    .password(true)
                                    .hint_text(egui::RichText::new("Enter your password")),
                            );
                            responses.push(response);
                        }

                        for response in responses {
                            if response.changed() {
                                inner
                                    .controller_requests
                                    .insert(ControllerRequest::UpdateAIRequestConfig);
                                break;
                            }
                        }
                    }
                },
            );

            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                |ui| {
                    if connect_to_ai_server {
                        if let Some(PropertieValue::DownloadState(ref state)) = inner
                            .properties
                            .get(&PropertieKey::AIServerConnectionStatus)
                        {
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("Test connection")
                                            .size(16.0)
                                            .color(egui::Color32::BLACK),
                                    )
                                    .fill(match state {
                                        DownloadState::Done => egui::Color32::GREEN,
                                        DownloadState::ParseError => egui::Color32::GREEN,
                                        DownloadState::InProgress => egui::Color32::YELLOW,
                                        DownloadState::Failed(..) => egui::Color32::RED,
                                        DownloadState::Null => egui::Color32::RED,
                                        DownloadState::None => egui::Color32::GRAY,
                                    }),
                                )
                                .clicked()
                            {
                                inner
                                    .controller_requests
                                    .insert(ControllerRequest::TestAIServerConnection(true));
                            }

                            ui.separator();
                        }
                    }
                },
            );
        }
    }
}
