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

#[derive(serde::Deserialize, serde::Serialize, derivative::Derivative)]
#[derivative(Debug)]
pub struct AppData {
    pub request_config: Arc<RwLock<RequestConfig>>,

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
}
impl AppData {
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
            static_audio: StaticAudio::new(),
            card_list: vec![/*Card {
                    display_data: CardDisplayData {
                        request_config: None,
                        question_text:
                            "Libre Learning is for when you want to learn on the fly, right now."
                                .to_owned(),
                        context_text: "<?readme/>".to_owned(),
                        label_text: "".to_owned(),
                        placeholder_text: "".to_owned(),
                        audio_item: None,
                        image_item: None,
                        image_index: 0usize,
                    },
                    meta_data: CardMetaData {
                        id: u16::MAX,
                        timestamps: Vec::new(),
                        scores: Vec::new(),
                    },
                }*/],
            space_repetition_model: SpacedRepetition::default(),
            card_download: None::<CardItem>,
            download_item_test: None,
        }
    }
}
