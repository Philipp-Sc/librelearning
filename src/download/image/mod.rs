use egui_extras::RetainedImage;

use super::super::app::RequestConfig;

use super::DownloadItem;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct ImageItem {
    download_item: DownloadItem,
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
        }
    }

    pub fn get_image(&mut self, request_config: &RequestConfig) -> Option<RetainedImage> {
        match self.download_item.fetch_download(request_config) {
            None => None, //RetainedImage::from_svg_bytes("svg",include_bytes!("../../assets/rust-logo.svg")).ok();
            Some(vec) => {
                return RetainedImage::from_image_bytes("img", &vec[..]).ok();
            }
        }
    }
}
