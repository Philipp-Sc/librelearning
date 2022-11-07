use std::sync::Arc;
use std::sync::RwLock;

use super::super::app::RequestConfig;
use crate::download::DownloadItem;
use crate::download::DownloadState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsDisplay {
    #[serde(skip)]
    pub request_config: Arc<RwLock<RequestConfig>>,

    pub auto_play_audio: bool,
    pub enable_sounds: bool,
    pub featch_new_card_at_threshold: bool,
    pub show_fetch_new_card_button: bool,
    pub add_new_card_threshold: f32,
    pub spelling_correction_threshold: usize,
    pub ignore_sentence_punctuation_symbols: bool,
    pub match_ascii: bool,
    pub match_case: bool,

    pub my_endpoint: String,
    pub my_account: String,
    pub my_password: String,

    #[serde(skip)]
    pub reset_app_requested: bool,

    #[serde(skip)]
    download_item_test: Option<DownloadItem>,
}

impl SettingsDisplay {
    pub fn set_request_config(&mut self) {
        if let Ok(ref mut conf) = self.request_config.write() {
            conf.endpoint = self.my_endpoint.to_owned();
            conf.user_name = self.my_account.to_owned();
            conf.password = self.my_password.to_owned();
        }
    }
}

impl Default for SettingsDisplay {
    fn default() -> Self {
        Self {
            request_config: Arc::new(RwLock::new(RequestConfig::default())),
            auto_play_audio: false,
            enable_sounds: true,
            featch_new_card_at_threshold: true,
            show_fetch_new_card_button: true,
            add_new_card_threshold: 66.6,
            spelling_correction_threshold: 1,
            ignore_sentence_punctuation_symbols: true,
            match_ascii: false,
            match_case: false,
            reset_app_requested: false,
            my_endpoint: "".to_string(),
            my_account: "".to_string(),
            my_password: "".to_string(),
            download_item_test: None,
        }
    }
}

impl super::Window for SettingsDisplay {
    fn name(&self) -> &'static str {
        "App Settings"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        if *open {
            let available_rect = ctx.available_rect();

            egui::Window::new(self.name())
                .fixed_rect(egui::Rect::from_min_size(
                    [available_rect.min.x + 5.0, available_rect.min.y + 40.0].into(),
                    [available_rect.max.x - 20.0, available_rect.max.y - 60.0].into(),
                ))
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

impl super::View for SettingsDisplay {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(
            egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
            |ui| {
                let response1 = ui.add(
                    egui::TextEdit::singleline(&mut self.my_endpoint)
                        .hint_text(egui::RichText::new("Enter your HTTP endpoint")),
                );
                let response2 = ui.add(
                    egui::TextEdit::singleline(&mut self.my_account)
                        .hint_text(egui::RichText::new("Enter your user name (optional)")),
                );
                let response3 = ui.add(
                    egui::TextEdit::singleline(&mut self.my_password)
                        .password(true)
                        .hint_text(egui::RichText::new("Enter your password (optional)")),
                );
                if response1.changed() || response2.changed() || response3.changed() {
                    self.set_request_config();
                }
            },
        );

        ui.with_layout(
            egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
            |ui| {
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Test connection")
                                .size(16.0)
                                .color(egui::Color32::BLACK),
                        )
                        .fill(
                            match self
                                .download_item_test
                                .as_ref()
                                .map(|x| x.get_download_state())
                            {
                                Some(DownloadState::Done) => egui::Color32::GREEN,
                                Some(DownloadState::ParseError) => egui::Color32::GREEN,
                                Some(DownloadState::InProgress) => egui::Color32::YELLOW,
                                Some(DownloadState::Failed(..)) => egui::Color32::RED,
                                Some(DownloadState::Null) => egui::Color32::RED,
                                None | Some(DownloadState::None) => egui::Color32::GRAY,
                            },
                        ),
                    )
                    .clicked()
                {
                    if let Ok(conf) = self.request_config.read() {
                        let mut download_item_test = DownloadItem::new("");
                        download_item_test.fetch_download(&conf);
                        self.download_item_test = Some(download_item_test);
                    }
                }
            },
        );

        ui.with_layout(
            egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
            |ui| {
                ui.checkbox(
                    &mut self.featch_new_card_at_threshold,
                    egui::RichText::new("Auto add new cards").size(16.0),
                );

                ui.add(
                    egui::Slider::new(&mut self.add_new_card_threshold, 0.0..=100.0)
                        .text(egui::RichText::new("at progress pct.").size(16.0)),
                );

                ui.separator();

                ui.checkbox(
                    &mut self.show_fetch_new_card_button,
                    egui::RichText::new("Show add new card button (🎉)").size(16.0),
                );

                ui.separator();

                ui.add(
                    egui::Slider::new(&mut self.spelling_correction_threshold, 0..=10)
                        .text(egui::RichText::new("Permitted spelling mistakes").size(16.0)),
                );

                ui.checkbox(
                    &mut self.ignore_sentence_punctuation_symbols,
                    egui::RichText::new("Ignore sentence punctuation symbols").size(16.0),
                );

                ui.checkbox(
                    &mut self.match_ascii,
                    egui::RichText::new("Match ASCII").size(16.0),
                );

                ui.checkbox(
                    &mut self.match_case,
                    egui::RichText::new("Match case (Lowercase/Uppercase)").size(16.0),
                );

                ui.separator();

                ui.checkbox(
                    &mut self.auto_play_audio,
                    egui::RichText::new("Auto play audio").size(16.0),
                );
                ui.checkbox(
                    &mut self.enable_sounds,
                    egui::RichText::new("Enable sounds").size(16.0),
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
            },
        );

        ui.with_layout(
            egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
            |ui| {
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Reset app").size(16.0), //.color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    self.reset_app_requested = true;
                }
            },
        );
    }
}
