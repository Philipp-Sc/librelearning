/*
#[cfg(not(target_arch = "wasm32"))]
use std::io::Cursor;

#[cfg(not(target_arch = "wasm32"))]
use rodio::{source::Source, Decoder, OutputStream};
*/

// TODO: update to fetch raw audio bytes, then play as in static_audio. remove HtmlAudioElement

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct AudioItem {
    audio_url: String,

    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    audio_element: Option<web_sys::HtmlAudioElement>,

    #[cfg(not(target_arch = "wasm32"))]
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    audio_data: Option<bytes::Bytes>,
}

impl Default for AudioItem {
    fn default() -> Self {
        AudioItem::new("")
    }
}

impl AudioItem {
    pub fn new(url: &str) -> AudioItem {
        AudioItem {
            audio_url: url.to_owned(),

            #[cfg(target_arch = "wasm32")]
            audio_element: None,

            #[cfg(not(target_arch = "wasm32"))]
            audio_data: None::<bytes::Bytes>,
        }
    }

    pub fn play(&mut self) {
        #[cfg(target_arch = "wasm32")]
        self.play_audio_web();

        /*#[cfg(not(target_arch = "wasm32"))]
        self.play_audio_native();*/
    }

    #[cfg(target_arch = "wasm32")]
    fn play_audio_web(&mut self) {
        match &self.audio_element {
            Some(audio_element) => {
                audio_element.play().ok();
            }
            None => match web_sys::HtmlAudioElement::new_with_src(&self.audio_url) {
                Ok(audio_element) => {
                    self.audio_element = Some(audio_element);
                    self.play_audio_web();
                }
                Err(_) => {}
            },
        }
    }
    /*
        #[cfg(not(target_arch = "wasm32"))]
        fn play_audio_native(&mut self) {
            //  BackendSpecificError { description: "ALSA function 'snd_pcm_open' failed with error 'EHOSTDOWN: Host is down'" }
            // TODO: create docker container with alsa support.

            let content: Option<std::io::Cursor<bytes::Bytes>>;

            if self.audio_data.is_none() {
                let request = ehttp::Request::get(&self.audio_url);
                let response = ehttp::fetch_blocking(&request);
                match response {
                    Ok(response) => match response.ok {
                        true => {
                            self.audio_data = Some(bytes::Bytes::from(response.bytes));
                            content = Some(Cursor::new(self.audio_data.as_ref().unwrap().clone()));
                        }
                        false => {
                            content = None;
                        }
                    },
                    Err(_) => {
                        content = None;
                    }
                }
            } else {
                content = Some(Cursor::new(self.audio_data.as_ref().unwrap().clone()));
            }
            match content {
                Some(content) => {
                    // Get a output stream handle to the default physical sound device
                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                    // Decode that sound file into a source
                    let source = Decoder::new(content).unwrap();
                    // Play the sound directly on the device
                    stream_handle.play_raw(source.convert_samples()).ok();
                }
                None => {
                    // error
                }
            }
        }
    */
}
