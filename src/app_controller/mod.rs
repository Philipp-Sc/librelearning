pub mod model_controller;
pub mod view_controller;

use model_controller::data_model::AppData;
use model_controller::ModelController;
use view_controller::view_model_controller::view_model::ViewModel;
use view_controller::ViewController;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AppController {
    model_controller: ModelController,
    view_controller: ViewController,
}
impl AppController {
    pub fn restore(&mut self) {
        let view_data: ViewModel = self.view_controller.view_model_controller.get_data();
        self.model_controller.restore(&view_data);
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let view_data: ViewModel = self.view_controller.view_model_controller.get_data();

        self.view_controller.update(ctx, _frame, &view_data);

        self.model_controller.update(&view_data);
    }
}

impl Default for AppController {
    fn default() -> Self {
        Self {
            model_controller: ModelController::default(),
            view_controller: ViewController::default(),
        }
    }
}
