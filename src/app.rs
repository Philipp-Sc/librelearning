use egui_extras::RetainedImage;

use super::download::audio::AudioItem;
use super::download::card::CardItem;
use super::download::image::ImageItem;
use super::static_audio::{StaticAudio, StaticSounds};

use js_sys::Date;

use super::spaced_repetition::*;
use std::collections::HashMap;

use super::windows::card::*;
use super::windows::review::*;
use super::windows::settings::*;
use super::windows::*;

use std::ops::*;
use std::sync::Arc;
use std::sync::RwLock;

// show popup when
// - card downloaded
// - card download failed.

// fetch next card button
// - color Green when enabled
// - popup text when clicked and disabled. "Learn a little more. You need more percent points, to be able to add a new card."

// set deck name, select deck in settings.

// format IL chinese card deck to right format.

// TODO: configure TSL for server later.

// then add telegram bot functionality.

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CardDisplayData {
    #[serde(skip)]
    request_config: Option<Arc<RwLock<RequestConfig>>>,

    question_text: String,
    context_text: String,
    label_text: String,
    placeholder_text: String,
    audio_item: Option<AudioItem>,
    image_item: Option<ImageItem>,
}

impl CardDisplayData {
    pub fn set_request_config(&mut self, request_config: &Arc<RwLock<RequestConfig>>) {
        if self.request_config.is_none() {
            self.request_config = Some(request_config.clone());
        }
    }

    pub fn get_question(&self) -> String {
        // Translate this sentence // What is the next word? // Pronounce this sentence
        self.question_text.to_owned()
    }
    pub fn get_context(&self) -> String {
        // Terima kasih, selamat idul fitri. // Where are you from? // What is your name?
        self.context_text.to_owned()
    }
    pub fn get_label(&self, ignore_punctuation_symbols: bool, match_case: bool) -> String {
        // Thank you, happy eid. // Kamu dari mana? // Nama kamu apa?
        let mut label = self.label_text.to_owned();
        if ignore_punctuation_symbols {
            label = label.replace(&['?', '(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
        }
        if !match_case {
            label = label.to_lowercase();
        }
        label
    }
    pub fn get_input_field_placeholder(&self) -> String {
        // Type the English translation // Type the Indonesian translation // Type what you hear
        self.placeholder_text.to_owned()
    }

    pub fn get_image(&mut self) -> Option<RetainedImage> {
        if let Some(ref mut image_item) = &mut self.image_item {
            if let Some(ref request_config) = self.request_config {
                if let Ok(conf) = request_config.read() {
                    return image_item.get_image(&conf);
                }
            }
        }
        None
    }

    pub fn play_audio(&mut self) -> bool {
        if let Some(audio_item) = &mut self.audio_item {
            if let Some(ref request_config) = self.request_config {
                if let Ok(conf) = request_config.read() {
                    return audio_item.play(&conf);
                }
            }
        }
        false
    }
    pub fn has_audio(&self) -> bool {
        self.audio_item.is_some()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CardMetaData {
    pub id: u16, // max value is 65553, switch to u32 if to small. (let me know if you got that many cards)
    pub timestamps: Vec<f64>,
    pub scores: Vec<bool>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Card {
    pub display_data: CardDisplayData,
    meta_data: CardMetaData,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CardFile {
    question_text: String,
    context_text: String,
    label_text: String,
    placeholder_text: String,
    audio_item: Option<String>,
    image_item: Option<String>,
    id: u16,
}

impl Card {
    pub fn parse(json_str: &str) -> Option<Self> {
        match serde_json::from_str::<CardFile>(json_str) {
            Err(_) => None,
            Ok(v) => Some(Self {
                display_data: CardDisplayData {
                    request_config: None,
                    question_text: v.question_text,
                    context_text: v.context_text,
                    label_text: v.label_text,
                    placeholder_text: v.placeholder_text,
                    audio_item: v.audio_item.map(|x| AudioItem::new(&x)),
                    image_item: v.image_item.map(|x| ImageItem::new(&x)),
                },
                meta_data: CardMetaData {
                    id: v.id,
                    timestamps: Vec::new(),
                    scores: Vec::new(),
                },
            }),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RequestConfig {
    pub endpoint: String,
    pub user_name: String,
    pub password: String,
}
impl RequestConfig {
    pub fn is_initialized(&self) -> bool {
        if self.endpoint.len() == 0 {
            return false;
        }
        true
    }
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            endpoint: "".to_string(),
            user_name: "".to_string(),
            password: "".to_string(),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct LibreLearningApp {
    settings_display: SettingsDisplay,

    #[serde(skip)]
    card_display: CardDisplay,

    #[serde(skip)]
    review_display: ReviewDisplay,

    #[serde(skip)]
    static_audio: StaticAudio,

    show_settings: bool,
    show_review: bool,

    check_review_done_requested: bool,

    progress: f32,
    user_text_input: String,
    text_input_has_focus: bool,

    handle_next_requested: bool,
    card_list: Vec<Card>,

    #[serde(skip)]
    space_repetition_model: SpacedRepetition,
    card_download: Option<CardItem>,

    new_card_requested: bool,
}

impl Default for LibreLearningApp {
    fn default() -> Self {
        Self {
            settings_display: SettingsDisplay::default(),
            card_display: CardDisplay::default(),
            review_display: ReviewDisplay::default(),
            static_audio: StaticAudio::new(),
            show_settings: false,
            show_review: false,
            check_review_done_requested: false,
            progress: 0.0,
            user_text_input: "".to_owned(),
            text_input_has_focus: false,
            handle_next_requested: false,
            card_list: vec![Card {
                display_data: CardDisplayData {
                    request_config: None,
                    question_text: "Translate this sentence".to_owned(),
                    context_text: "Terima kasih, selamat idul fitri.".to_owned(),
                    label_text: "Thank you, happy eid.".to_owned(),
                    placeholder_text: "Type the English translation".to_owned(),
                    audio_item: Some(AudioItem::new("audio/prompt.wav")),
                    image_item: Some(ImageItem::new(
                        "images/DALL_E_Terima_kasih__Selamat_idul_fitri.png",
                    )),
                },
                meta_data: CardMetaData {
                    id: 0,
                    timestamps: Vec::new(),
                    scores: Vec::new(),
                },
            }],
            space_repetition_model: SpacedRepetition::default(),
            card_download: None::<CardItem>,
            new_card_requested: false,
        }
    }
}

impl LibreLearningApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        super::static_fonts::setup_custom_fonts(&cc.egui_ctx);

        let mut visuals = egui::Visuals::dark();

        visuals.extreme_bg_color = egui::Color32::from_gray(42);
        visuals.selection.bg_fill = egui::Color32::GREEN; // for progress color
        visuals.selection.stroke.color = egui::Color32::BLACK; // for progress text

        cc.egui_ctx.set_visuals(visuals);

        super::is_ready();

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            let mut previous_instance: LibreLearningApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            previous_instance.settings_display.set_request_config();

            // loaded cards get the current request_config, (freshly fetched cards also get this).
            for i in 0..previous_instance.card_list.len() {
                previous_instance.card_list[i]
                    .display_data
                    .set_request_config(&previous_instance.settings_display.request_config);
            }
            return previous_instance;
        }

        Default::default()
    }

    fn take_action(&mut self) {
        if self.settings_display.reset_app_requested {
            *self = Default::default();
        }

        if self.handle_next_requested {
            if self.settings_display.auto_play_audio {
                self.card_list[0].display_data.play_audio();
            }
            self.handle_next_requested = false;
        }

        if self.check_review_done_requested {
            if !self.show_review {
                self.next();
                self.check_review_done_requested = false;
            }
        }
        if self.new_card_requested {
            if self.try_add_new_card() {
                self.new_card_requested = false;
                self.check_review_done_requested = true;
            }
        }
    }
    fn evaluate_review(&mut self) -> bool {
        self.card_list[0].meta_data.timestamps.push(Date::now());
        if self.settings_display.ignore_punctuation_symbols {
            self.user_text_input = self
                .user_text_input
                .replace(&['?', '(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
        }
        if !self.settings_display.match_case {
            self.user_text_input = self.user_text_input.to_lowercase();
        }
        let score = edit_distance::edit_distance(
            &self.user_text_input,
            &self.card_list[0].display_data.get_label(
                self.settings_display.ignore_punctuation_symbols,
                self.settings_display.match_case,
            ),
        ) <= self.settings_display.spelling_correction_threshold;
        self.card_list[0].meta_data.scores.push(score);
        score
    }

    fn try_add_new_card(&mut self) -> bool {
        let card_id: u16 = self.card_list.len() as u16;
        if let Some(card) = self.fetch_card(card_id) {
            self.card_list.push(card);
            return true;
        }
        false
    }

    fn fetch_card(&mut self, card_id: u16) -> Option<Card> {
        if let Some(ref mut card_download) = &mut self.card_download {
            return match card_download.fetch_card(&self.settings_display.request_config) {
                Some(c) => {
                    self.card_download = None;
                    Some(c)
                }
                None => None,
            };
        } else {
            let card_url = format!("card_{}.json", card_id);
            let mut card_download = CardItem::new(&card_url);
            return match card_download.fetch_card(&self.settings_display.request_config) {
                Some(c) => Some(c),
                None => {
                    self.card_download = Some(card_download);
                    None
                }
            };
        }
    }

    fn next(&mut self) {
        self.user_text_input = "".to_owned();

        assert!(self.card_list.len() > 0);

        let mut list: HashMap<u16, u64> = HashMap::new();
        let mut count_score_true: usize = 0;
        let mut count_score_false: usize = 0;
        let mut count_score_undefined: usize = 0;
        let mut timeout_list: HashMap<u16, bool> = HashMap::new();
        for i in 0..self.card_list.len() {
            {
                let last_three_indices =
                    (0..self.card_list[i].meta_data.scores.len()).rev().take(3);
                count_score_true += last_three_indices
                    .clone()
                    .filter_map(|x| match self.card_list[i].meta_data.scores[x] {
                        true => Some(true),
                        _ => None,
                    })
                    .count();
                count_score_false += last_three_indices
                    .filter_map(|x| match self.card_list[i].meta_data.scores[x] {
                        false => Some(false),
                        _ => None,
                    })
                    .count();
                count_score_undefined += if self.card_list[i].meta_data.scores.len() == 0 {
                    1usize
                } else {
                    0usize
                };
            }
            {
                let time: f64 = estimate_next_session_timestamp(
                    &mut self.space_repetition_model,
                    &mut self.card_list[i].meta_data,
                );
                // let date = Date::new(&wasm_bindgen::JsValue::from_f64(time));
                /*
                super::log(&format!(
                    "Card_ID: {}\nCard_Question: {}\n{:?}\n{:?}",
                    self.card_list[i].meta_data.id,
                    self.card_list[i].display_data.get_context(),
                    time,
                    date.to_iso_string()
                ));
                */
                list.insert(self.card_list[i].meta_data.id, time.trunc() as u64);
            }
            {
                // check if there are at least 3 timestamps within the last 7 minutes
                let seven_min_ago: f64 = Date::now() - 1000.0 * 60.0 * 7.0;
                timeout_list.insert(
                    self.card_list[i].meta_data.id,
                    self.card_list[i]
                        .meta_data
                        .timestamps
                        .iter()
                        .filter(|&t| t >= &seven_min_ago)
                        .count()
                        >= 3usize,
                );
            }
        }
        self.progress = count_score_true as f32
            / ((count_score_true + count_score_false + count_score_undefined) as f32);

        let reviewed_card_id = self.card_list[0].meta_data.id;
        self.card_list
            .sort_unstable_by_key(|card| list.get(&card.meta_data.id));

        // maintaining previous sort, and sort by timeout_list
        // cards in timeout are moved to the end of the card_list.
        self.card_list
            .sort_by_key(|card| timeout_list.get(&card.meta_data.id).map(|x| *x as u8));

        // this prevents the same card being shown twice
        if self.card_list.len() > 1 && self.card_list[0].meta_data.id == reviewed_card_id {
            self.card_list.swap(0, 1);
        }

        self.handle_next_requested = true;

        //super::log(&format!("{:?}",(count_score_true,count_score_true+count_score_false,count_score_undefined)));
    }
}

impl eframe::App for LibreLearningApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        assert!(self.card_list.len() > 0);

        self.take_action();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.add(
                    egui::widgets::ProgressBar::new(self.progress)
                        .desired_width(ui.available_width() - 36.0),
                );

                let response = ui.add(
                    egui::Button::new(egui::RichText::new("⚙").size(18.0))
                        .fill(egui::Color32::BLACK),
                );

                if response.clicked() {
                    self.show_settings = !self.show_settings;
                }
            });

            if self.show_settings {
                self.settings_display.show(ctx, &mut self.show_settings);
            } else {
                self.review_display.show(
                    ctx,
                    &mut self.show_review,
                    &self.card_list[0].display_data.get_label(
                        self.settings_display.ignore_punctuation_symbols,
                        self.settings_display.match_case,
                    ),
                    &self.user_text_input,
                    &self.card_list[0].meta_data.scores.last().unwrap_or(&false),
                );

                self.card_display
                    .ui(ui, &mut self.card_list[0].display_data);

                let _text_input_response = ui.add_sized(
                    ui.available_size().sub(
                        [
                            0.0,
                            if self.settings_display.show_fetch_new_card_button {
                                90.0
                            } else {
                                40.0
                            },
                        ]
                        .into(),
                    ),
                    egui::TextEdit::multiline(&mut self.user_text_input)
                        .frame(false)
                        .cursor_at_end(true)
                        .hint_text(egui::RichText::new(
                            self.card_list[0].display_data.get_input_field_placeholder(),
                        )) // Type the Indonesian translation // Type what you hear
                        .text_color(egui::Color32::WHITE)
                        .font(egui::FontId::proportional(20.0))
                        //.lock_focus(true)
                        .desired_width(f32::INFINITY),
                );

                // take up the available space, but not the last n pixels.
                let mut available_space: egui::Vec2 = ui.available_size();
                available_space.y = available_space.y
                    - if self.settings_display.show_fetch_new_card_button {
                        90.0
                    } else {
                        50.0
                    };
                ui.allocate_space(available_space);

                if self.settings_display.show_fetch_new_card_button {
                    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        let check = ui.add(egui::Button::new(egui::RichText::new("🎉").size(40.0)));
                        if check.clicked() {
                            self.new_card_requested = true;
                        }
                    });
                }

                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::BOTTOM).with_main_justify(true),
                    |ui| {
                        let check = ui.add(egui::Button::new(egui::RichText::new("↩").size(30.0)));
                        if check.clicked() {
                            let score = self.evaluate_review();
                            if self.settings_display.enable_sounds {
                                if score {
                                    self.static_audio
                                        .play_audio(&StaticSounds::BeginningOfLine)
                                        .ok();
                                } else {
                                    self.static_audio
                                        .play_audio(&StaticSounds::ServiceLogout)
                                        .ok();
                                }
                            }

                            self.check_review_done_requested = true;
                            self.show_review = true;
                        }
                    },
                );
            }
        });
    }
}
