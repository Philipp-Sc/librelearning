use std::sync::{Arc, Mutex};

use egui_extras::RetainedImage;

enum Download {
    None,
    InProgress,
    Done(ehttp::Result<ehttp::Response>),
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct ImageItem {
    image_url: String,
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    download: Arc<Mutex<Download>>,
}

impl Default for ImageItem {
    fn default() -> Self {
        ImageItem::new("")
    }
}

impl ImageItem {
    pub fn new(url: &str) -> ImageItem {
        ImageItem {
            image_url: url.to_owned(),
            download: Arc::new(Mutex::new(Download::None)),
        }
    }

    pub fn get_image(&mut self) -> Option<RetainedImage> {
        {
            let download: &Download = &self.download.lock().unwrap();
            match download {
                Download::None => {}
                Download::InProgress => {
                    return None;
                }
                Download::Done(response) => match response {
                    Err(_) => {
                        return None;
                    }
                    Ok(response) => {
                        return Some(
                            RetainedImage::from_image_bytes("my-logo.jpeg", &response.bytes[..])
                                .unwrap(),
                        );
                    }
                },
            };
        }
        self.download_image();
        None
    }
    fn download_image(&mut self) {
        let request = ehttp::Request::get(&self.image_url);

        let download_store = self.download.clone();
        *download_store.lock().unwrap() = Download::InProgress;
        ehttp::fetch(request, move |response| {
            *download_store.lock().unwrap() = Download::Done(response);
        });
    }
}
