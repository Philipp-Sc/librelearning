use egui_extras::RetainedImage;

use super::download::audio::AudioItem;
use super::download::card::CardItem;
use super::download::image::ImageItem;
use super::static_audio::{StaticAudio, StaticSounds};

use js_sys::Date;

use super::spaced_repetition::*;
use std::collections::HashMap;

use super::windows::alert::*;
use super::windows::card::*;
use super::windows::review::*;
use super::windows::settings::*;
use super::windows::*;

use std::ops::*;
use std::sync::Arc;
use std::sync::RwLock;

use any_ascii::any_ascii;

use crate::download::DownloadState;

// first card when no card has yet been fetched. create tutorial card.

// format IL chinese card deck to right format. // use DALLE to generate PICTURES

// error warning when auto fetching -> disable auto fetching

// save app state and tag it, enable loading/switching saved app states, that way can use multiple different decks. (only meta data needs to be saved, saving card data is convinient)
// possibly have random manager, to switch between the tags for learning.

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
    pub fn get_label(
        &self,
        ignore_sentence_punctuation_symbols: bool,
        match_ascii: bool,
        match_case: bool,
    ) -> String {
        // Thank you, happy eid. // Kamu dari mana? // Nama kamu apa?
        let mut label = self.label_text.to_owned();
        if ignore_sentence_punctuation_symbols {
            label = label.replace(&['?', '(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
        }
        if match_ascii {
            label = any_ascii(&label);
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
        // starts with http(s)://  and ends with no /
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
    alert_display: AlertDisplay,

    #[serde(skip)]
    static_audio: StaticAudio,

    show_settings: bool,
    show_review: bool,
    alert_text: Option<String>,

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

    #[serde(skip)]
    play_sound: Option<StaticSounds>,
}

impl Default for LibreLearningApp {
    fn default() -> Self {
        Self {
            settings_display: SettingsDisplay::default(),
            card_display: CardDisplay::default(),
            review_display: ReviewDisplay::default(),
            alert_display: AlertDisplay::default(),
            static_audio: StaticAudio::new(),
            show_settings: false,
            show_review: false,
            alert_text: None,
            check_review_done_requested: false,
            progress: 0.0,
            user_text_input: "".to_owned(),
            text_input_has_focus: false,
            handle_next_requested: false,
            card_list: vec![Card {
                display_data: CardDisplayData {
                    request_config: None,
                    question_text: "For when you really just want to learn a new language right now!".to_owned(),
                    context_text: "To get started open the ⚙ Settings and enter the connection details to your material.\n\n\nAfter that you can:\n\n- fetch new cards (🎉)\n\n- start reviewing your material (↩)".to_owned(),
                    label_text: "".to_owned(),
                    placeholder_text: "".to_owned(),
                    audio_item: None,
                    image_item: None,
                },
                meta_data: CardMetaData {
                    id: u16::MAX,
                    timestamps: Vec::new(),
                    scores: Vec::new(),
                },
            }],
            space_repetition_model: SpacedRepetition::default(),
            card_download: None::<CardItem>,
            new_card_requested: false,
            play_sound: None,
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

        let mut previous_instance: LibreLearningApp;

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            previous_instance = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            previous_instance.settings_display.set_request_config();

            // loaded cards get the current request_config, (freshly fetched cards also get this).
            for i in 0..previous_instance.card_list.len() {
                previous_instance.card_list[i]
                    .display_data
                    .set_request_config(&previous_instance.settings_display.request_config);
            }
            return previous_instance;
        } else {
            previous_instance = Default::default()
        }
        previous_instance
    }

    fn take_action(&mut self) {
        if self.settings_display.reset_app_requested {
            *self = Default::default();
        }
        if let Some(static_sound) = &self.play_sound.clone() {
            self.play_sound = None;
            self.static_audio.play_audio(&static_sound).ok();
        }
        if self.card_list.len() == 2 && self.card_list[1].meta_data.id == u16::MAX {
            // Deleting the inital demo card.
            self.card_list.pop();
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
            match self.try_add_new_card() {
                Ok(_) => {
                    self.new_card_requested = false;
                    self.play_sound = Some(StaticSounds::BeginningOfLine);
                    self.handle_next_requested = true; // because new card is next.
                }
                Err(err) => {
                    match err {
                        DownloadState::Failed(..)
                        | DownloadState::ParseError
                        | DownloadState::Null => {
                            self.new_card_requested = false;
                            self.alert_text = Some(format!("{:?}", err));
                            self.card_download = None;
                        }
                        _ => {} // InProgress
                    }
                }
            }
        }
    }
    fn evaluate_review(&mut self) -> bool {
        self.card_list[0].meta_data.timestamps.push(Date::now());
        if self.settings_display.ignore_sentence_punctuation_symbols {
            self.user_text_input = self
                .user_text_input
                .replace(&['?', '(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
        }

        if self.settings_display.match_ascii {
            self.user_text_input = any_ascii(&self.user_text_input);
        }
        if !self.settings_display.match_case {
            self.user_text_input = self.user_text_input.to_lowercase();
        }
        let score = edit_distance::edit_distance(
            &self.user_text_input,
            &self.card_list[0].display_data.get_label(
                self.settings_display.ignore_sentence_punctuation_symbols,
                self.settings_display.match_ascii,
                self.settings_display.match_case,
            ),
        ) <= self.settings_display.spelling_correction_threshold;
        self.card_list[0].meta_data.scores.push(score);
        score
    }

    fn try_add_new_card(&mut self) -> Result<(), DownloadState> {
        let card_id: u16 =
            if self.card_list.len() == 1 && self.card_list[0].meta_data.id == u16::MAX {
                // Deleting the inital demo card.
                0u16
            } else {
                self.card_list.len() as u16
            };

        match self.fetch_card(card_id) {
            Ok(card) => {
                self.card_list.insert(0, card);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn fetch_card(&mut self, card_id: u16) -> Result<Card, DownloadState> {
        if let Some(ref mut card_download) = &mut self.card_download {
            return match card_download.fetch_card(&self.settings_display.request_config) {
                Ok(Some(c)) => {
                    self.card_download = None;
                    Ok(c)
                }
                Err(err) => Err(err),
                Ok(None) => Err(DownloadState::ParseError),
            };
        } else {
            let card_url = format!("card_{}.json", card_id);
            let mut card_download = CardItem::new(&card_url);
            return match card_download.fetch_card(&self.settings_display.request_config) {
                Ok(Some(c)) => Ok(c),
                Ok(None) => {
                    self.card_download = Some(card_download);
                    Err(DownloadState::ParseError)
                }
                Err(err) => {
                    self.card_download = Some(card_download);
                    Err(err)
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

        if self.settings_display.featch_new_card_at_threshold
            && self.progress * 100.0 >= self.settings_display.add_new_card_threshold
        {
            self.new_card_requested = true;
        }

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
                self.alert_display.show(ctx, &mut self.alert_text);
                self.review_display.show(
                    ctx,
                    &mut self.show_review,
                    &self.card_list[0].display_data.get_label(
                        self.settings_display.ignore_sentence_punctuation_symbols,
                        self.settings_display.match_ascii,
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
                        let check = ui.add(
                            egui::Button::new(egui::RichText::new("🎉").size(40.0)).fill(
                                egui::Color32::from_rgb(
                                    (255.0 - (255.0 * self.progress)) as u8,
                                    (255.0 * self.progress) as u8,
                                    (255.0 * 0.9) as u8,
                                ),
                            ),
                        );
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
                                    self.play_sound = Some(StaticSounds::BeginningOfLine);
                                } else {
                                    self.play_sound = Some(StaticSounds::ServiceLogout);
                                }
                            }

                            self.check_review_done_requested = true;
                            self.show_review = true;
                        }
                    },
                );
            }
        });
        // to save CPU ressources egui does not always automaically update the UI
        // the following line ensures egui updates the UI at least every 250ms (human reaction time).
        ctx.request_repaint_after(std::time::Duration::from_millis(250));
        // TODO: this can be optimized by checking if there are any background tasks (e.g. Downloads InProgress)
        // if no the duration can be longer
    }
}
