pub mod card;
pub mod review;
pub mod settings;

/// Something to view in a window
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait Window {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

/// Something to view
pub trait ReviewWindow {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool, label: &str, text: &str, score: &bool);
}

/// Something to view in a window
pub trait CardView {
    fn ui(&mut self, ui: &mut egui::Ui, card_display_data: &mut super::app::CardDisplayData);
}
