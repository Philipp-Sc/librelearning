use crate::app_controller::view_controller::display::DisplayViewModel;
use crate::app_controller::view_controller::display::WindowViewModel;
use crate::app_controller::view_controller::view_model_controller::view_model::ControllerRequest;
use crate::app_controller::view_controller::view_model_controller::view_model::DisplayKind;
use crate::app_controller::view_controller::view_model_controller::view_model::{
    PropertieKey, PropertieValue,
};
use crate::app_controller::ViewModel;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SaveLoadSettingsDisplay {}

impl Default for SaveLoadSettingsDisplay {
    fn default() -> Self {
        Self {}
    }
}

impl WindowViewModel for SaveLoadSettingsDisplay {
    fn show(&mut self, ctx: &egui::Context, view_model: &ViewModel) {
        if let Ok(mut inner) = view_model.inner.lock() {
            if !(DisplayKind::SaveLoadSettingsDisplay == inner.display_kind) {
                return;
            }
        }
        let available_rect = ctx.available_rect();
        egui::Window::new("Save/Load Settings")
            .fixed_rect(egui::Rect::from_min_size(
                [available_rect.min.x + 5.0, available_rect.min.y + 140.0].into(),
                [available_rect.max.x - 20.0, available_rect.max.y].into(),
            ))
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui, view_model);
            });
    }
}

impl DisplayViewModel for SaveLoadSettingsDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, view_model: &ViewModel) {


        ui.with_layout(
            egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
            |ui| {

                ui.label(
                    egui::RichText::new("Checkpoints:") 
                        .strong() 
                        .size(16.0),
                );

                ui.separator();

            });
        ui.with_layout(
            egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
            |ui| {

                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Save").size(16.0), //.color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    if let Ok(mut inner) = view_model.inner.lock() {
                        inner.controller_requests.insert(ControllerRequest::SaveCheckpoint);
                    }
                }
                ui.separator();
            },
        );

        ui.with_layout(
            egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
            |ui| {
                    let mut checkpoints_copy = None;

                    view_model.get_property(&PropertieKey::Checkpoints, |val| {
                        if let PropertieValue::VecString(ref checkpoints) = val {
                            checkpoints_copy = Some(checkpoints.clone());
                        }
                    });

                    if let Some(checkpoints) = checkpoints_copy {
                        view_model.update_property(&PropertieKey::SelectedCheckpoint, |val| {
                            if let PropertieValue::String(ref mut selected_checkpoint) = val {
                                for title in &checkpoints {
                                    ui.radio_value(selected_checkpoint, title.to_owned(), &*title);
                                }
                            }
                        });
                    }
            },
        );
        ui.with_layout(
            egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
            |ui| {
                ui.separator();
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Load").size(16.0), //.color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    if let Ok(mut inner) = view_model.inner.lock() {
                        inner.controller_requests.insert(ControllerRequest::LoadCheckpoint);
                    }
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Delete").size(16.0), //.color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    if let Ok(mut inner) = view_model.inner.lock() {
                        inner.controller_requests.insert(ControllerRequest::DeleteCheckpoint);
                    }
                }

                ui.separator();

                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Import").size(16.0), //.color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    if let Ok(mut inner) = view_model.inner.lock() { 
                    }
                }

                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Export").size(16.0), //.color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    if let Ok(mut inner) = view_model.inner.lock() { 
                    }
                }


                ui.separator();
               
                
            },
        );
        ui.with_layout(
            egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
            |ui| {

                ui.label(
                    egui::RichText::new("Danger Zone:") 
                        .strong() 
                        .size(16.0),
                );

                ui.separator();

            });


        ui.with_layout(
            egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
            |ui| {
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Reset App").size(16.0), //.color(egui::Color32::BLACK),
                    ))
                    .clicked()
                {
                    if let Ok(mut inner) = view_model.inner.lock() {
                        inner
                            .controller_requests
                            .insert(ControllerRequest::ResetApp);
                    }
                }
            });
    }
}
