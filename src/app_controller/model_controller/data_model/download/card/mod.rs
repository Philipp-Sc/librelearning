use super::DownloadItem;
use crate::app_controller::model_controller::data_model::card_model::Card;
use crate::app_controller::model_controller::data_model::request_model::RequestConfig;

use std::sync::Arc;
use std::sync::RwLock;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct CardItem {
    download_item: DownloadItem,
}

impl Default for CardItem {
    fn default() -> Self {
        CardItem::new("")
    }
}

impl CardItem {
    pub fn new(url: &str) -> CardItem {
        CardItem {
            download_item: DownloadItem::new(url),
        }
    }

    pub fn fetch_card(
        &mut self,
        request_config: &Arc<RwLock<RequestConfig>>,
    ) -> Result<Option<Card>, super::DownloadState> {
        match if let Ok(conf) = request_config.read() {
            self.download_item.try_fetch_download(&conf)
        } else {
            Err(super::DownloadState::None)
        } {
            Err(err) => Err(err),
            Ok(vec) => {
                match std::str::from_utf8(&vec[..]) {
                    Ok(v) => {
                        let mut card = Card::parse(v);
                        if let Some(ref mut c) = card {
                            c.display_data.set_request_config(request_config);
                        }
                        Ok(card)
                    }
                    Err(_e) => Ok(None), //  panic!("Invalid UTF-8 sequence: {}", e),
                }
            }
        }
    }
}
