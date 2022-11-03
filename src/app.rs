use egui_extras::RetainedImage;

use super::audio::AudioItem;
use super::image::ImageItem;
use super::static_audio::{StaticAudio, StaticSounds};

use js_sys::Date;

use super::spaced_repetition::*;
use std::collections::HashMap;

use super::windows::settings::*;
use super::windows::Window;

// TODO: show error, if incorrect review.
// TODO: option forgiving label == input evaluation.

// TODO: replace test data.

// TODO: make sure audio finishes playing before starting another.

// TODO authentication, only be able to access my own files. (image url/audio url), consider cryptic link or login.

// client makes request, sending all card_ids it has, and requests a new card.
// server answers with some(new card) or none.

// create simple server that hosts files from IL chinese.
// then add telegram bot functionality.

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CardDisplay {
    question_text: String,
    context_text: String,
    label_text: String,
    placeholder_text: String,
    audio_item: Option<AudioItem>,
    image_item: Option<ImageItem>,
}

impl CardDisplay {
    fn get_question(&self) -> String {
        // Translate this sentence // What is the next word? // Pronounce this sentence
        self.question_text.to_owned()
    }
    fn get_context(&self) -> String {
        // Terima kasih, selamat idul fitri. // Where are you from? // What is your name?
        self.context_text.to_owned()
    }
    fn get_label(&self) -> String {
        // Thank you, happy eid. // Kamu dari mana? // Nama kamu apa?
        self.label_text.to_owned()
    }
    fn get_input_field_placeholder(&self) -> String {
        // Type the English translation // Type the Indonesian translation // Type what you hear
        self.placeholder_text.to_owned()
    }

    fn get_image(&mut self) -> Option<RetainedImage> {
        if let Some(ref mut image_item) = &mut self.image_item {
            image_item.get_image()
        } else {
            None
        }
    }

    fn play_audio_item(&mut self) {
        if let Some(audio_item) = &mut self.audio_item {
            audio_item.play();
        }
    }
    fn has_audio_item(&self) -> bool {
        self.audio_item.is_some()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CardMeta {
    pub id: u16, // max value is 65535 // switch to u32 if to small.
    pub timestamps: Vec<f64>,
    pub scores: Vec<bool>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Card {
    display_data: CardDisplay,
    meta_data: CardMeta,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct LibreLearningApp {
    settings: SettingsWindow,

    #[serde(skip)]
    static_audio: StaticAudio,

    show_settings: bool,

    progress: f32,
    user_text_input: String,
    text_input_has_focus: bool,

    is_next: bool,
    card_list: Vec<Card>,

    #[serde(skip)]
    space_repetition_model: SpacedRepetition, // not saved, it will be restored from the card_list.
}

impl Default for LibreLearningApp {
    fn default() -> Self {
        Self {
            settings: SettingsWindow::default(),
            static_audio: StaticAudio::new(),
            show_settings: false,
            progress: 0.0,
            user_text_input: "".to_owned(), 
            text_input_has_focus: false,
            is_next: false,
            card_list: vec![Card {
                                display_data: CardDisplay {
                                    question_text: "Translate this sentence".to_owned(), 
                                    context_text: "Terima kasih, selamat idul fitri.".to_owned(),
                                    label_text: "Thank you, happy eid.".to_owned(),
                                    placeholder_text: "Type the English translation".to_owned(),
                                    audio_item: Some(AudioItem::new("https://dobrian.github.io/cmp/topics/sample-recording-and-playback-with-web-audio-api/freejazz.wav")),
                                    image_item: Some(ImageItem::new("https://raw.githubusercontent.com/Philipp-Sc/librelearning/main/assets/DALL_E_Terima_kasih__Selamat_idul_fitri.png")),
                                },
                                meta_data: CardMeta {
                                    id: 0,
                                    timestamps: Vec::new(),
                                    scores: Vec::new(),
                                },
                            },
                            Card {
                                display_data: CardDisplay {
                                    question_text: "What is the missing word?".to_owned(), 
                                    context_text: "______ kasih, selamat idul fitri.".to_owned(),
                                    label_text: "Terima".to_owned(),
                                    placeholder_text: "Type the missing word".to_owned(),
                                    audio_item: None,
                                    image_item: None,
                                },
                                meta_data: CardMeta {
                                    id: 1,
                                    timestamps: Vec::new(),
                                    scores: Vec::new(),
                                },
                            },
                            Card {
                                display_data: CardDisplay {
                                    question_text: "What is the missing word?".to_owned(), 
                                    context_text: "Terima kasih, _______ idul fitri.".to_owned(),
                                    label_text: "selamat".to_owned(),
                                    placeholder_text: "Type the missing word".to_owned(),
                                    audio_item: None,
                                    image_item: None,
                                },
                                meta_data: CardMeta {
                                    id: 2,
                                    timestamps: Vec::new(),
                                    scores: Vec::new(),
                                },
                            }
                            ],
            space_repetition_model: SpacedRepetition::default(),
        }
    }
}

impl LibreLearningApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut visuals = egui::Visuals::dark();

        visuals.extreme_bg_color = egui::Color32::from_gray(42);
        visuals.selection.bg_fill = egui::Color32::GREEN; // for progress color
        visuals.selection.stroke.color = egui::Color32::BLACK; // for progress text

        cc.egui_ctx.set_visuals(visuals);

        super::is_ready();

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
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
                let date = Date::new(&wasm_bindgen::JsValue::from_f64(time));
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

        self.is_next = true;

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
        if self.settings.reset_app {
            *self = Default::default();
        }
        assert!(self.card_list.len() > 0);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                self.settings.show(ctx, &mut self.show_settings);

                ui.add(
                    egui::widgets::ProgressBar::new(self.progress)
                        .desired_width(ui.available_width() - 36.0)
                        .show_percentage(),
                );

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new("⚙").size(18.0))
                            .fill(egui::Color32::BLACK),
                    )
                    .clicked()
                {
                    self.show_settings = !self.show_settings;
                }
            });

            ui.allocate_space(egui::Vec2 { x: 0.0, y: 20.0 });
            ui.heading(egui::RichText::new(
                self.card_list[0].display_data.get_question(),
            ));
            ui.separator();

            ui.allocate_space(egui::Vec2 { x: 0.0, y: 40.0 });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let play_audio_text = if self.card_list[0].display_data.has_audio_item() {
                    "🔊"
                } else {
                    "🔇"
                };

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(play_audio_text).size(20.0))
                            .frame(false),
                    )
                    .clicked()
                {
                    self.card_list[0].display_data.play_audio_item();
                }

                ui.label(
                    egui::RichText::new(self.card_list[0].display_data.get_context())
                        .color(egui::Color32::WHITE)
                        .size(20.0),
                );
            });

            ui.allocate_space(egui::Vec2 { x: 0.0, y: 20.0 });
            ui.vertical_centered_justified(|ui| {
                self.card_list[0].display_data.get_image().map(
                    |img| img.show(ui), /*_size(ui, egui::Vec2 { x: 200.0, y: 200.0 }))*/
                );
                // note this is not efficient
            });

            ui.allocate_space(egui::Vec2 { x: 0.0, y: 40.0 });
            let text_input_response = ui.add(
                egui::TextEdit::multiline(&mut self.user_text_input)
                    //.frame(false)
                    .cursor_at_end(true)
                    .hint_text(egui::RichText::new(
                        self.card_list[0].display_data.get_input_field_placeholder(),
                    )) // Type the Indonesian translation // Type what you hear
                    .text_color(egui::Color32::WHITE)
                    .font(egui::FontId::proportional(20.0))
                    //.lock_focus(true)
                    .desired_width(f32::INFINITY),
            );

            let mut available_space: egui::Vec2 = ui.available_size();
            available_space.y = available_space.y - 60.0;
            ui.allocate_space(available_space);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if self.settings.add_new_card_threshold <= self.progress {
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("+ Add New Card")
                                    .size(20.0)
                                    .color(egui::Color32::BLACK),
                            )
                            .fill(egui::Color32::GREEN),
                        )
                        .clicked()
                    {
                        if self.settings.enable_sounds {
                            self.static_audio
                                .play_audio(&StaticSounds::MessageNewInstant);
                        }
                        // TODO trigger new card to load. // and call next so the new card gets scheduled.
                    }
                }
                ui.vertical_centered_justified(|ui| {
                    let check = ui.button(egui::RichText::new("↩").size(20.0));
                    if check.clicked() {
                        self.card_list[0].meta_data.timestamps.push(Date::now());
                        if self.user_text_input == self.card_list[0].display_data.get_label() {
                            self.static_audio.play_audio(&StaticSounds::BeginningOfLine); // MessageNewInstant
                            if self.settings.enable_sounds {
                                self.card_list[0].meta_data.scores.push(true);
                            }
                        } else {
                            // TODO change color for button in red blinking.
                            if self.settings.enable_sounds {
                                self.static_audio.play_audio(&StaticSounds::ServiceLogout);
                            }
                            self.card_list[0].meta_data.scores.push(false);
                        }
                        self.next();
                    }
                });
            });
        });

        if self.settings.auto_play_audio && self.is_next {
            self.card_list[0].display_data.play_audio_item();
            self.is_next = false;
        }
    }
}
