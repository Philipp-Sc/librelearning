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

#[derive(Debug)]
pub enum DownloadState {
    Null,
    None,
    InProgress,
    Done,
    Failed(String),
    ParseError,
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
                Download::Failed(response, _) => DownloadState::Failed(
                    response
                        .as_ref()
                        .ok()
                        .map(|x| x.status.to_string())
                        .unwrap_or("Error".to_string()),
                ),
                Download::None => DownloadState::None,
                Download::InProgress => DownloadState::InProgress,
                Download::Done(_response) => DownloadState::Done,
            }
        }
    }

    pub fn try_fetch_download(
        &mut self,
        request_config: &RequestConfig,
    ) -> Result<Vec<u8>, DownloadState> {
        {
            let mutex = self.download.lock();
            if let Err(_) = &mutex {
                return Err(DownloadState::None);
            }
            let download: &Download = &mutex.unwrap();
            match download {
                Download::Failed(response, _timestamp) => {
                    return Err(DownloadState::Failed(
                        response
                            .as_ref()
                            .ok()
                            .map(|x| x.status.to_string())
                            .unwrap_or("Error".to_string()),
                    ));
                }
                Download::None => {}
                Download::InProgress => {
                    return Err(DownloadState::InProgress);
                }
                Download::Done(response) => {
                    return Ok(response.bytes.clone());
                }
            };
        }
        if !request_config.is_initialized() {
            return Err(DownloadState::Null);
        }
        self.download(request_config);
        Err(DownloadState::None)
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
        let credentials;

        let mut headers = Vec::new();
        if request_config.user_name.len() > 0 && request_config.password.len() > 0 {
            credentials = Credentials::new(&request_config.user_name, &request_config.password)
                .as_http_header();

            headers.push(("Authorization", credentials.as_str()));
        }

        let request = ehttp::Request {
            headers: ehttp::headers(&headers[..]),
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
