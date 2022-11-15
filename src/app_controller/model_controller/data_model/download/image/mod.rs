use egui_extras::RetainedImage;

use crate::app_controller::model_controller::data_model::request_model::RequestConfig;

use super::DownloadItem;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct ImageItem {
    pub download_item: DownloadItem,

    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub retained_image: Option<Arc<Mutex<RetainedImage>>>,
}

impl Default for ImageItem {
    fn default() -> Self {
        ImageItem::new("")
    }
}

impl ImageItem {
    pub fn new(url: &str) -> ImageItem {
        ImageItem {
            download_item: DownloadItem::new(url),
            retained_image: None,
        }
    }

    pub fn get_image(
        &mut self,
        request_config: &RequestConfig,
    ) -> Option<Arc<Mutex<RetainedImage>>> {
        if self.retained_image.is_some() {
            return self.retained_image.clone();
        }
        match self.download_item.fetch_download(request_config) {
            None => None, //RetainedImage::from_svg_bytes("svg",include_bytes!("../../assets/rust-logo.svg")).ok();
            Some(vec) => {
                self.retained_image = RetainedImage::from_image_bytes("img", &vec[..])
                    .ok()
                    .map(|x| Arc::new(Mutex::new(x)));
                return self.retained_image.clone();
            }
        }
    }
}
