// Everything NON-UI

use std::sync::RwLock;
use std::sync::{Arc, Mutex};

pub mod card_model;
pub mod download;
pub mod request_model;
pub mod spaced_repetition;
pub mod static_audio;
pub mod static_fonts;

use card_model::Card;
use card_model::CardDisplayData;
use card_model::CardMetaData;
use download::card::CardItem;
use spaced_repetition::SpacedRepetition;
use static_audio::StaticAudio;

use crate::app_controller::model_controller::data_model::download::DownloadItem;
use crate::app_controller::model_controller::data_model::download::DownloadState;
use crate::app_controller::model_controller::data_model::request_model::RequestConfig;

use egui_extras::RetainedImage;

#[derive(serde::Deserialize, serde::Serialize, derivative::Derivative)]
#[derivative(Debug)]
pub struct AppData {
    pub request_config: Arc<RwLock<RequestConfig>>,
    pub ai_request_config: Arc<RwLock<RequestConfig>>,

    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub static_audio: StaticAudio,

    pub card_list: Vec<Card>,

    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub space_repetition_model: SpacedRepetition,

    pub card_download: Option<CardItem>,

    #[serde(skip)]
    download_item_test: Option<DownloadItem>,

    #[serde(skip)]
    ai_download_item_test: Option<DownloadItem>,
}
impl AppData {

    pub fn play_audio_of_card(&mut self, index: usize) -> bool {
        if self.card_list.len() <= index {
            return false;
        }
        if !self.card_list[index].display_data.play_audio(self.request_config.clone()) {
            return false;
        }else{
            return true;
        }
        
    }

    pub fn load_audio_of_card(&mut self, index: usize){
        if self.card_list.len() <= index {
            return;
        }

        let current_card = &mut self.card_list[index];
        if current_card.display_data.has_audio() {
            current_card.display_data.load_audio(self.request_config.clone());
        }        
    }

    pub fn card_has_image(&mut self, index: usize) -> bool {
        if self.card_list.len() <= index {
            return false;
        }
        let current_card = &mut self.card_list[index];

        if current_card.display_data.has_image() {
             return true;
        } 
        false
    }

    pub fn get_image_of_card(&mut self, index: usize) -> Option<Arc<Mutex<RetainedImage>>> {
        if self.card_list.len() <= index {
            return None;
        }
        let current_card = &mut self.card_list[index];

        if current_card.display_data.has_image() {
             return current_card.display_data.get_image(self.request_config.clone());
        } 
        None
    }

    // TODO
    pub fn get_dalle_image_for_card(&mut self, index: usize) -> Option<Arc<Mutex<RetainedImage>>> {
        if self.card_list.len() <= index {
            return None;
        }
        let current_card = &mut self.card_list[index];

        // first iterate over image_items to look for "dalle/?query="

        if let Some(ref image_item) = current_card.display_data.image_item {
            for i in 0..image_item.len() {
                if image_item[i].download_item.url.contains("dalle?") {
                    current_card.display_data.image_index = i;
                    return current_card.display_data.get_image(self.ai_request_config.clone());
                }
            }
        }


        let qs = qstring::QString::new(vec![ 
            ("context", format!("\"{}\"",current_card.display_data.get_context())), 
            ("label", format!("\"{}\"",current_card.display_data.get_label())), 
         ]);
 
        current_card.display_data.add_image_from_url(&format!("dalle?{}",qs));
        current_card.display_data.get_image(self.ai_request_config.clone())
    }

    pub fn custom_server_connection_status(&mut self) -> DownloadState {
        self.download_item_test
            .as_ref()
            .map(|x| x.get_download_state())
            .unwrap_or(DownloadState::Null)
    }

    pub fn test_custom_server_connection(&mut self) {
        if let Ok(conf) = self.request_config.read() {
            let mut download_item_test = DownloadItem::new("card_0.json");
            download_item_test.fetch_download(&conf);
            self.download_item_test = Some(download_item_test);
        }
    }

    pub fn ai_server_connection_status(&mut self) -> DownloadState {
        self.ai_download_item_test
            .as_ref()
            .map(|x| x.get_download_state())
            .unwrap_or(DownloadState::Null)
    }

    pub fn test_ai_server_connection(&mut self) {
        if let Ok(conf) = self.ai_request_config.read() {
            let qs = qstring::QString::new(vec![ 
                ("context", format!("\"{}\"","http connection test")), 
                ("label", format!("\"{}\"","status 200")), 
             ]);

            let mut ai_download_item_test = DownloadItem::new(&format!("dalle?{}",qs));
            ai_download_item_test.fetch_download(&conf);
            self.ai_download_item_test = Some(ai_download_item_test);
        }
    }

    pub fn try_add_new_card(&mut self) -> Result<(), DownloadState> {
        let card_id: u16 = self.card_list.len() as u16;

        match self.fetch_card(card_id) {
            Ok(card) => {
                self.card_list.insert(0, card);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn fetch_card(&mut self, card_id: u16) -> Result<Card, DownloadState> {
        {
            if let Some(ref mut card_download) = &mut self.card_download {
                return match card_download.fetch_card(&self.request_config) {
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
                return match card_download.fetch_card(&self.request_config) {
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
        Err(DownloadState::Failed("Missing Config".to_string()))
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            request_config: Arc::new(RwLock::new(RequestConfig::default())),
            ai_request_config: Arc::new(RwLock::new(RequestConfig::default())),
            static_audio: StaticAudio::new(),
            card_list: Vec::new(),
            space_repetition_model: SpacedRepetition::default(),
            card_download: None::<CardItem>,
            download_item_test: None,
            ai_download_item_test: None,
        }
    }
}
