use std::sync::{Arc, Mutex};
   

#[cfg(not(target_arch = "wasm32"))]
use std::io::BufReader; 
 
use std::io::{Write, Read, Cursor};
 
use rodio::{Decoder, OutputStream, source::Source};


use egui_extras::RetainedImage;


// TODO authentication, only be able to access my own files.

// server sends Card hash
// client checks if card available in store
// client returns YES or NO
// NO: server sends Card
// YES: -
// Card loaded.
// client sends training result
// client sends next card request


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LibreLearningApp {
    heading_txt: String,
    header_txt: String,
    label_txt: String,
    label_txt_edit: String,
    hint_text_input_txt: String,
    progress_bar: f32,
    
    #[serde(skip)]
    audio_item: Option<AudioItem>, // todo add serialize/deserialize to only save the URL.


    #[serde(skip)]
    image_item: Option<ImageItem>,

    // this how you opt-out of serialization of a member
    //#[serde(skip)]
    //value: f32,
}

enum Download {
    None,
    InProgress,
    Done(ehttp::Result<ehttp::Response>),
}

pub struct ImageItem {
    image_url: String,  
    download: Arc<Mutex<Download>>,
}

impl ImageItem {


    fn new(url: &str) -> ImageItem {
        ImageItem {
            image_url: url.to_owned(),
            download:  Arc::new(Mutex::new(Download::None)),
        }
    }

    fn get_image(&mut self) -> Option<RetainedImage> {
 
        {
            let download: &Download = &self.download.lock().unwrap();
            match download {
                Download::None => {}
                Download::InProgress => { return None;}
                Download::Done(response) => match response {
                    Err(err) => {
                        return None;
                    }
                    Ok(response) => {
                        
                        return Some(RetainedImage::from_image_bytes(
                            "my-logo.jpeg",
                            &response.bytes[..],
                        )
                        .unwrap());
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

pub struct AudioItem {
    audio_url: String, 
    download: Arc<Mutex<Download>>,  
    bytes: Vec<u8>,
}
impl AudioItem {

    fn new(url: &str) -> AudioItem {
        AudioItem {
            audio_url: url.to_owned(),
            download:  Arc::new(Mutex::new(Download::None)),  
            bytes: Vec::new(),
        }
    }

    fn get_audio_bytes(&mut self) {
  
        {
            let download: &Download = &self.download.lock().unwrap();
            match download {
                Download::None => {}
                Download::InProgress => {}
                Download::Done(response) => match response {
                    Err(err) => { 
                    }
                    Ok(response) => {
                        match response.ok {
                            true => { 
                                self.bytes.append(&mut response.bytes.clone()); 
                            },
                            false => { 
                            }
                        }
                    }
                },
            }; 
        } 
        self.download_audio(); 
    }

    fn download_audio(&mut self) {
        let request = ehttp::Request::get(&self.audio_url);

        let download_store = self.download.clone();
        *download_store.lock().unwrap() = Download::InProgress; 
        ehttp::fetch(request, move |response| {
            *download_store.lock().unwrap() = Download::Done(response); 
        });
    } 

    fn play(&mut self) {

        if self.bytes.len() > 0 { 
            let content = Cursor::new(bytes::Bytes::from(self.bytes.clone()));
            // Get a output stream handle to the default physical sound device
            let (_stream, stream_handle) = OutputStream::try_default().unwrap(); 
            // Decode that sound file into a source
            let source = Decoder::new(content).unwrap();
            // Play the sound directly on the device
            stream_handle.play_raw(source.convert_samples());  
        }else{
            self.get_audio_bytes();
        }
    }
     
}

impl Default for LibreLearningApp {
    fn default() -> Self {
        Self {
            heading_txt: "Translate this sentence".to_owned(), 
            header_txt: "Terima kasih, selamat idul fitri.".to_owned(),
            label_txt: "Thank you, happy eid.".to_owned(),
            label_txt_edit: "".to_owned(),
            hint_text_input_txt: "Type the English translation".to_owned(),
            audio_item: Some(AudioItem::new("https://dobrian.github.io/cmp/topics/sample-recording-and-playback-with-web-audio-api/freejazz.wav")),
            progress_bar: 0.0,
            image_item: Some(ImageItem::new("https://avatars.githubusercontent.com/u/16901158")),
        }
    }
}

// web_sys::AudioContext (clone example and try to load an audio file from https request)
// TODO: Progress Bar
// TODO: Image Display (maybe this first)
// TODO: Audio File Display,Play,Stop.
// TODO: Audio Recording.

impl LibreLearningApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

}




impl eframe::App for LibreLearningApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { heading_txt, header_txt, label_txt, label_txt_edit, hint_text_input_txt, audio_item, progress_bar, .. } = self;

        egui::CentralPanel::default().show(ctx, |ui| {

            ctx.set_visuals(egui::Visuals::dark());
            let mut visuals = ui.visuals_mut();
            visuals.extreme_bg_color = egui::Color32::from_gray(42); 
            //visuals.selection.stroke.color = egui::Color32::GREEN; 
            visuals.selection.bg_fill = egui::Color32::GREEN; 
            //visuals.faint_bg_color = egui::Color32::GREEN; 
  
            ui.add(egui::widgets::ProgressBar::new(*progress_bar)); 

            ui.allocate_space(egui::Vec2{ x: 0.0, y: 20.0,});
            ui.heading(egui::RichText::new(heading_txt.to_owned())); // What is the next word? // Pronounce this sentence
            ui.separator();


            ui.allocate_space(egui::Vec2{ x: 0.0, y: 40.0,}); 
           
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button(egui::RichText::new("🔊").size(20.0)).clicked() { 

                    if let Some(audio_item) = audio_item {
                        audio_item.play();
                    }
                    
                }
 
                ui.label(egui::RichText::new(header_txt.to_owned()).color(egui::Color32::WHITE).size(20.0)); 
            });
              
            if let Some(ref mut image_item) = &mut self.image_item {
                if let Some(image) = image_item.get_image() {
                    image.show(ui);
                }
            }
 
     

            ui.allocate_space(egui::Vec2{ x: 0.0, y: 40.0,}); 
            ui.add(
                egui::TextEdit::multiline(&mut *label_txt_edit)
                    .frame(false)
                    .hint_text(egui::RichText::new(hint_text_input_txt.to_owned())) // Type the Indonesian translation // Type what you hear
                    .text_color(egui::Color32::WHITE)
                    .font(egui::FontId::proportional(20.0))
                    .lock_focus(true)
                    .desired_width(f32::INFINITY) 
            );

            let mut available_space: egui::Vec2 = ui.available_size();
            available_space.y = available_space.y - 40.0;
            ui.allocate_space(available_space);

            
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.vertical_centered_justified(|ui| {

                    let response = ui.button(egui::RichText::new("↩").size(20.0)); 
                    if response.clicked() {   
                        if label_txt_edit == label_txt {   
                            *progress_bar += 0.1
                        }else{
                            *progress_bar -= 0.1
                        }
                    }
                });
            });
 
           
        });
 
    }
}
