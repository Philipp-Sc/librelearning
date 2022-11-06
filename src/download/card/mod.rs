use super::super::app::RequestConfig;
use super::super::app::*;
use super::DownloadItem;

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

    pub fn fetch_card(&mut self, request_config: &Arc<RwLock<RequestConfig>>) -> Option<Card> {
        match if let Ok(conf) = request_config.read() {
            self.download_item.fetch_download(&conf)
        } else {
            None
        } {
            None => None,
            Some(vec) => {
                match std::str::from_utf8(&vec[..]) {
                    Ok(v) => {
                        let mut card = Card::parse(v);
                        if let Some(ref mut c) = card {
                            c.display_data.set_request_config(request_config);
                        }
                        card
                    }
                    Err(_e) => None, //  panic!("Invalid UTF-8 sequence: {}", e),
                }
            }
        }
    }
}
