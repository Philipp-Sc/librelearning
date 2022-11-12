use crate::app_controller::model_controller::data_model::request_model::RequestConfig;

use crate::app_controller::model_controller::data_model::static_audio::StaticAudio;

use super::DownloadItem;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct AudioItem {
    download_item: DownloadItem,
}

impl Default for AudioItem {
    fn default() -> Self {
        AudioItem::new("")
    }
}

impl AudioItem {
    pub fn new(url: &str) -> AudioItem {
        AudioItem {
            download_item: DownloadItem::new(url),
        }
    }

    pub fn load(&mut self, request_config: &RequestConfig) {
        self.download_item.fetch_download(request_config);
    }

    pub fn play(&mut self, request_config: &RequestConfig) -> bool {
        match self.download_item.fetch_download(request_config) {
            None => false,
            Some(vec) => {
                StaticAudio::play_audio_from_bytes(&vec[..]).ok();
                true
            }
        }
    }
}
