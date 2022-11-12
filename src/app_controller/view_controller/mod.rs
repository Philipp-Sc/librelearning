mod display;
pub mod view_model_controller;

use super::model_controller::data_model::AppData;
use view_model_controller::view_model::DisplayKind;
use view_model_controller::view_model::ViewModel;
use view_model_controller::ViewModelController;

use display::component::alert::AlertDisplay;
use display::component::app::AppDisplay;
use display::component::review::ReviewDisplay;
use display::component::settings::api::APISettingsDisplay;
use display::component::settings::options::OptionsSettingsDisplay;
use display::component::settings::save_load::SaveLoadSettingsDisplay;
use display::component::settings::SettingsDisplay;
use display::WindowViewModel;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ViewController {
    pub view_model_controller: ViewModelController,
}
impl ViewController {
    pub fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        view_model: &ViewModel,
    ) {
        AppDisplay::default().show(ctx, view_model);

        SettingsDisplay::default().show(ctx, view_model);

        APISettingsDisplay::default().show(ctx, view_model);
        OptionsSettingsDisplay::default().show(ctx, view_model);
        SaveLoadSettingsDisplay::default().show(ctx, view_model);
        AlertDisplay::default().show(ctx, view_model);
        ReviewDisplay::default().show(ctx, view_model);

        // to save CPU ressources egui does not always automaically update the UI
        // the following line ensures egui updates the UI at least every 250ms (average human reaction time).
        ctx.request_repaint_after(std::time::Duration::from_millis(250));
        // TODO: this can be optimized by checking if there are any background tasks (e.g. ControllerRequests)
        // if no the duration can be longer
    }
}

impl Default for ViewController {
    fn default() -> Self {
        Self {
            view_model_controller: ViewModelController::default(),
        }
    }
}
