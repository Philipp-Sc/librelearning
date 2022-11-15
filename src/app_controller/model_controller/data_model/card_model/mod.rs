use super::download::audio::AudioItem;
use super::download::image::ImageItem;
use super::request_model::RequestConfig;
use any_ascii::any_ascii;
use std::sync::Arc;
use std::sync::RwLock;

use egui_extras::RetainedImage;
use std::sync::Mutex;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CardDisplayData { 

    pub question_text: String,
    pub context_text: String,
    pub label_text: String,
    pub placeholder_text: String,
    pub audio_item: Option<AudioItem>,
    pub image_item: Option<Vec<ImageItem>>,

    #[serde(skip)]
    pub image_index: usize,
}

impl CardDisplayData { 

    pub fn get_question(&self) -> String {
        // Translate this sentence // What is the next word? // Pronounce this sentence
        self.question_text.to_owned()
    }
    pub fn get_context(&self) -> String {
        // Terima kasih, selamat idul fitri. // Where are you from? // What is your name?
        self.context_text.to_owned()
    }
    pub fn get_label(
        &self, /*
               ignore_sentence_punctuation_symbols: bool,
               match_ascii: bool,
               match_case: bool,*/
    ) -> String {
        // Thank you, happy eid. // Kamu dari mana? // Nama kamu apa?
        /*let mut label = */
        self.label_text.to_owned() //;
                                   /*
                                   if ignore_sentence_punctuation_symbols {
                                       label = label.replace(&['?', '(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
                                   }
                                   if match_ascii {
                                       label = any_ascii(&label);
                                   }
                                   if !match_case {
                                       label = label.to_lowercase();
                                   }
                                   label*/
    }
    pub fn get_input_field_placeholder(&self) -> String {
        // Type the English translation // Type the Indonesian translation // Type what you hear
        self.placeholder_text.to_owned()
    }

    /*
    pub fn show_image(&mut self, ui: &mut egui::Ui) {
        if let Some(ref mut image_item) = &mut self.image_item {
            if let Some(retained_image) = &image_item[self.image_index].retained_image {
                retained_image.show_size(ui, egui::Vec2 { x: 200.0, y: 200.0 });
            }
        }
    }*/

    pub fn add_image_from_url(&mut self,url: &str) {
        let image = ImageItem::new(url);
        if let Some(ref mut image_item) = self.image_item {
            image_item.push(image);
            self.image_index = image_item.len()-1;
        }else{
            self.image_item = Some(vec![image]);
            self.image_index = 0;
        }

    }

    pub fn has_image(&mut self) -> bool {
        self.image_item.is_some()
    }

    pub fn get_image(&mut self, request_config: Arc<RwLock<RequestConfig>>) -> Option<Arc<Mutex<RetainedImage>>> {
        if let Some(ref mut image_item) = &mut self.image_item { 
            if let Ok(conf) = request_config.read() {
                return image_item[self.image_index].get_image(&conf);
            } 
        }
        None
    }

    pub fn play_audio(&mut self, request_config: Arc<RwLock<RequestConfig>>) -> bool {
        if let Some(audio_item) = &mut self.audio_item { 
            if let Ok(conf) = request_config.read() {
                return audio_item.play(&conf);
            } 
        }
        false
    }
    pub fn load_audio(&mut self, request_config: Arc<RwLock<RequestConfig>>) {
        if let Some(audio_item) = &mut self.audio_item { 
            if let Ok(conf) = request_config.read() {
                audio_item.load(&conf);
            } 
        }
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
    pub meta_data: CardMetaData,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CardFile {
    question_text: String,
    context_text: String,
    label_text: String,
    placeholder_text: String,
    audio_item: Option<String>,
    image_item: Option<Vec<String>>,
    id: u16,
}

impl Card {
    pub fn parse(json_str: &str) -> Option<Self> {
        match serde_json::from_str::<CardFile>(json_str) {
            Err(_) => None,
            Ok(v) => Some(Self {
                display_data: CardDisplayData { 
                    question_text: v.question_text,
                    context_text: v.context_text,
                    label_text: v.label_text,
                    placeholder_text: v.placeholder_text,
                    audio_item: v.audio_item.map(|x| AudioItem::new(&x)),
                    image_item: v.image_item.map(|x| {
                        x.into_iter()
                            .map(|y| ImageItem::new(&y))
                            .collect::<Vec<ImageItem>>()
                    }),
                    image_index: 0usize,
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
