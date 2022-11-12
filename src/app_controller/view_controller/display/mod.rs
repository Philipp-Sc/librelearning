use super::view_model_controller::view_model::ViewModel;

pub mod component;

/// Something to view
pub trait DisplayViewModel {
    fn ui(&mut self, ui: &mut egui::Ui, view_model: &ViewModel);
}

/// Something to view
pub trait WindowViewModel {
    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, view_model: &ViewModel);
}
