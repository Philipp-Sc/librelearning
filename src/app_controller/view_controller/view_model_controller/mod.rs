pub mod view_model;
use view_model::ViewModel;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ViewModelController {
    view_model_data: ViewModel,
}
impl ViewModelController {
    pub fn get_data(&mut self) -> ViewModel {
        self.view_model_data.clone()
    }
}

impl Default for ViewModelController {
    fn default() -> Self {
        Self {
            view_model_data: ViewModel::default(),
        }
    }
}
