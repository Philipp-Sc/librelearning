pub mod audio;
pub mod card;
pub mod image;

use js_sys::Date;
use std::sync::{Arc, Mutex};

enum Download {
    None,
    InProgress,
    Done(ehttp::Response),
    Failed(ehttp::Result<ehttp::Response>, f64),
}

pub enum DownloadState {
    None,
    InProgress,
    Done,
    Failed,
}

use http_auth_basic::Credentials;

use super::app::RequestConfig;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct DownloadItem {
    url: String,
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    download: Arc<Mutex<Download>>,
}

impl Default for DownloadItem {
    fn default() -> Self {
        DownloadItem::new("")
    }
}

impl DownloadItem {
    pub fn new(url: &str) -> DownloadItem {
        DownloadItem {
            url: url.to_owned(),
            download: Arc::new(Mutex::new(Download::None)),
        }
    }
    pub fn get_download_state(&self) -> DownloadState {
        let mutex = self.download.lock();
        if let Err(_) = &mutex {
            DownloadState::InProgress
        } else {
            let download: &Download = &mutex.unwrap();
            match download {
                Download::Failed(..) => DownloadState::Failed,
                Download::None => DownloadState::None,
                Download::InProgress => DownloadState::InProgress,
                Download::Done(_response) => DownloadState::Done,
            }
        }
    }

    pub fn fetch_download(&mut self, request_config: &RequestConfig) -> Option<Vec<u8>> {
        {
            let mutex = self.download.lock();
            if let Err(_) = &mutex {
                return None;
            }
            let download: &Download = &mutex.unwrap();
            match download {
                Download::Failed(_, _timestamp) => {
                    return None;
                }
                Download::None => {}
                Download::InProgress => {
                    return None;
                }
                Download::Done(response) => {
                    return Some(response.bytes.clone());
                }
            };
        }
        if !request_config.is_initialized() {
            return None;
        }
        self.download(request_config);
        None
    }
    fn download(&mut self, request_config: &RequestConfig) {
        let credentials = Credentials::new(&request_config.user_name, &request_config.password);
        let credentials = credentials.as_http_header();

        let request = ehttp::Request {
            headers: ehttp::headers(&[("Authorization", &credentials)]),
            ..ehttp::Request::get(&format!("{}/{}", &request_config.endpoint, &self.url))
        };

        let download_store = self.download.clone();
        *download_store.lock().unwrap() = Download::InProgress;
        ehttp::fetch(request, move |response| {
            let timestamp: f64 = Date::now();

            *download_store.lock().unwrap() = match response {
                Err(_) => Download::Failed(response, timestamp),
                Ok(ref r) => {
                    if r.ok {
                        Download::Done(r.clone())
                    } else {
                        Download::Failed(response, timestamp)
                    }
                }
            };
        });
    }
}
