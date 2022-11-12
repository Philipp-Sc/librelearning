  
use crate::app_controller::AppController;
use crate::app_controller::model_controller::data_model::static_fonts::setup_custom_fonts;

// LOAD/SAVE/DELETE State. Enables Switching Decks, without losing everything.

// USE DALLE API! yay. // Ckeckbox Augment with DALL-E // -> SERVER API.  // Server (saves all image generations and prompts, so they might be reused.) // if enabled DalleImageItem::new(prompt).

// format IL chinese card deck to right format. // use DALLE to generate PICTURES

// error warning when auto fetching -> disable auto fetching

// save app state and tag it, enable loading/switching saved app states, that way can use multiple different decks. (only meta data needs to be saved, saving card data is convinient)
// possibly have random manager, to switch between the tags for learning.

// then add telegram bot functionality.

// Current language learning Apps function without adaption to the student i.e they use a pre planned lesson plan and then apply a spaced repetition algorithm to aid in the memorization process.
// In real live when you travel abroad you are immidiatly confronted with language challenges in a specific context, the situation you find yourself in.
// Traditional Apps in part can account for this by creating a specific learing schedule / lesson.

// This App is for when you just want to learn on the fly, right now!
// Context aware language learning, meant for students abroad.
//
// Tell librelearning what you want to learn and it will prepare the material for you. Only learn what you need, when you need it.
//
// Whisper -> Transcribe -> GPT-3 -> Generate Card -> DALL-E -> Generate Image -> Text-To-Speech -> Generate Audio.
 

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct LibreLearningApp {
    app_controller: AppController,
}

impl Default for LibreLearningApp {
    fn default() -> Self {
        Self {
            app_controller: AppController::default(),
        }
    }
}

impl LibreLearningApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        setup_custom_fonts(&cc.egui_ctx);

        let mut visuals = egui::Visuals::dark();

        visuals.extreme_bg_color = egui::Color32::from_gray(42);
        visuals.selection.bg_fill = egui::Color32::GREEN; // for progress color
        visuals.selection.stroke.color = egui::Color32::BLACK; // for progress text

        cc.egui_ctx.set_visuals(visuals);

        super::is_ready();

        let mut previous_instance: LibreLearningApp;

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            previous_instance = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            previous_instance.app_controller.restore();
        } else {
            previous_instance = Default::default()
        }
        previous_instance
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
        self.app_controller.update(ctx, _frame);
    }
}
