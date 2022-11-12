pub mod data_model;
use data_model::AppData;

use super::view_controller::view_model_controller::view_model::ControllerRequest;
use super::view_controller::view_model_controller::view_model::PropertieKey;
use super::view_controller::view_model_controller::view_model::PropertieValue;
use super::view_controller::view_model_controller::view_model::ViewModel;
use super::view_controller::view_model_controller::view_model::VolatilePropertieKey;
use super::view_controller::view_model_controller::view_model::VolatilePropertieValue;
use crate::app_controller::model_controller::data_model::download::DownloadState;
use crate::app_controller::model_controller::data_model::static_audio::StaticSounds;
use std::collections::HashSet;

use any_ascii::any_ascii;
use difference::{Changeset, Difference};
use js_sys::Date;

use crate::app_controller::model_controller::data_model::spaced_repetition::estimate_next_session_timestamp;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ModelController {
    app_data: AppData,
}
impl ModelController {
    pub fn restore(&mut self, view_model: &ViewModel) {
        // loaded cards get the current request_config, (freshly fetched cards also get this).
        for i in 0..self.app_data.card_list.len() {
            self.app_data.card_list[i]
                .display_data
                .set_request_config(&self.app_data.request_config);
        }

        let mut retained_controller_requests = HashSet::new();

        retained_controller_requests.insert(ControllerRequest::LoadAudio);
        retained_controller_requests.insert(ControllerRequest::LoadImage);

        if let Ok(mut inner) = view_model.inner.lock() {
            inner
                .controller_requests
                .extend(retained_controller_requests);
        }
    }
    pub fn update(&mut self, view_model: &ViewModel) {
        let mut controller_requests = HashSet::new();
        if let Ok(mut inner) = view_model.inner.lock() {
            controller_requests = inner.controller_requests.drain().collect();
        }
        if controller_requests.contains(&ControllerRequest::ResetApp) {
            if let Ok(mut inner) = view_model.inner.lock() {
                *inner = Default::default();
            }
            *self = Default::default();
            return;
        }

        let mut retained_controller_requests = HashSet::new();

        for request in controller_requests {
            match request {
                ControllerRequest::ResetApp => {
                    panic!()
                }
                ControllerRequest::Save => {}
                ControllerRequest::Load => {}
                ControllerRequest::Delete => {}
                ControllerRequest::UpdateRequestConfig => {
                    if let Ok(mut request_config) = self.app_data.request_config.write() {
                        if let Ok(mut inner) = view_model.inner.lock() {
                            if let Some(PropertieValue::String(ref endpoint)) =
                                inner.properties.get(&PropertieKey::CustomServerEndpoint)
                            {
                                request_config.endpoint = endpoint.to_owned();
                            }
                            if let Some(PropertieValue::String(ref user_name)) =
                                inner.properties.get(&PropertieKey::CustomServerUsername)
                            {
                                request_config.user_name = user_name.to_owned();
                            }
                            if let Some(PropertieValue::String(ref password)) =
                                inner.properties.get(&PropertieKey::CustomServerPassword)
                            {
                                request_config.password = password.to_owned();
                            }
                        }
                    }
                }
                ControllerRequest::TestCustomServerConnection(update) => {
                    if update {
                        self.app_data.test_custom_server_connection();
                        retained_controller_requests
                            .insert(ControllerRequest::TestCustomServerConnection(false));
                    } else {
                        if let Ok(mut inner) = view_model.inner.lock() {
                            if let Some(PropertieValue::DownloadState(ref mut status)) = inner
                                .properties
                                .get_mut(&PropertieKey::CustomServerConnectionStatus)
                            {
                                *status = self.app_data.custom_server_connection_status();

                                if let DownloadState::InProgress = status {
                                    retained_controller_requests.insert(
                                        ControllerRequest::TestCustomServerConnection(false),
                                    );
                                }
                            }
                        }
                    }
                }
                ControllerRequest::TestAIServerConnection => {}

                ControllerRequest::RefreshCard => {
                    if self.app_data.card_list.len() > 0 {
                        if let Ok(mut inner) = view_model.inner.lock() {
                            let current_card = &mut self.app_data.card_list[0];

                            inner.properties.insert(
                                PropertieKey::CardQuestion,
                                PropertieValue::String(current_card.display_data.get_question()),
                            );
                            inner.properties.insert(
                                PropertieKey::CardContext,
                                PropertieValue::String(current_card.display_data.get_context()),
                            );
                            inner.properties.insert(
                                PropertieKey::UserTextInput,
                                PropertieValue::String("".to_string()),
                            );
                            inner.properties.insert(
                                PropertieKey::UserTextInputHint,
                                PropertieValue::String(
                                    current_card.display_data.get_input_field_placeholder(),
                                ),
                            );
                            inner.properties.insert(
                                PropertieKey::CardHasAudio,
                                PropertieValue::Bool(current_card.display_data.has_audio()),
                            );

                            retained_controller_requests.insert(ControllerRequest::LoadImage);
                            retained_controller_requests.insert(ControllerRequest::LoadAudio);
                        }
                    }
                }
                ControllerRequest::FetchNewCard => {
                    match self.app_data.try_add_new_card() {
                        Ok(_) => {
                            //retained_controller_requests.insert(ControllerRequest::PlaySound(StaticSounds::BeginningOfLine));

                            if let Ok(mut inner) = view_model.inner.lock() {
                                retained_controller_requests.insert(ControllerRequest::RefreshCard);
                            }
                        }
                        Err(err) => match err {
                            DownloadState::Failed(..)
                            | DownloadState::ParseError
                            | DownloadState::Null => {

                                view_model
                                .insert_property(PropertieKey::Alert, PropertieValue::String(format!("{:?}", err))); 

                                self.app_data.card_download = None;

                                retained_controller_requests.insert(ControllerRequest::RefreshCard);

                            }
                            _ => {
                                retained_controller_requests
                                    .insert(ControllerRequest::FetchNewCard);
                            }
                        },
                    }
                }
                ControllerRequest::CheckReview => {
                    if self.app_data.card_list.len()==0{
                        return;
                    }

                    let mut spelling_correction_threshold: usize = 1;

                    let mut label = self.app_data.card_list[0].display_data.get_label();
                    let mut output = "".to_string();

                    if let Ok(mut inner) = view_model.inner.lock() {
                        if let Some(PropertieValue::String(ref user_text_input)) =
                            inner.properties.get(&PropertieKey::UserTextInput)
                        {
                            output.push_str(user_text_input);
                        }
                        if let Some(PropertieValue::Bool(ref val)) =
                            inner.properties.get(&PropertieKey::MatchASCII)
                        {
                            if *val {
                                label = any_ascii(&label);
                                output = any_ascii(&output);
                            }
                        }
                        if let Some(PropertieValue::Bool(ref val)) =
                            inner.properties.get(&PropertieKey::MatchCase)
                        {
                            if !*val {
                                label = label.to_lowercase();
                                output = output.to_lowercase();
                            }
                        }
                        if let Some(PropertieValue::Bool(ref val)) = inner
                            .properties
                            .get(&PropertieKey::IgnoreSentencePunctuationSymbols)
                        {
                            if *val {
                                label = label.replace(
                                    &['?', '(', ')', ',', '\"', '.', ';', ':', '\''][..],
                                    "",
                                );
                                output = output.replace(
                                    &['?', '(', ')', ',', '\"', '.', ';', ':', '\''][..],
                                    "",
                                );
                            }
                        }
                        if let Some(PropertieValue::Usize(ref val)) = inner
                            .properties
                            .get(&PropertieKey::SpellingCorrectionThreshold)
                        {
                            spelling_correction_threshold = *val;
                        }
                    }

                    let score = edit_distance::edit_distance(&output, &label)
                        <= spelling_correction_threshold;

                    self.app_data.card_list[0]
                        .meta_data
                        .timestamps
                        .push(Date::now());
                    self.app_data.card_list[0].meta_data.scores.push(score);

                    let changeset = Changeset::new(&label, &output, "");

                    if let Ok(mut inner) = view_model.inner.lock() {
                        inner
                            .properties
                            .insert(PropertieKey::ReviewScore, PropertieValue::Bool(score));
                        inner.volatile_properties.insert(
                            VolatilePropertieKey::Differences,
                            VolatilePropertieValue::Differences(changeset.diffs),
                        );
                    }

                    if score {
                        retained_controller_requests
                            .insert(ControllerRequest::PlaySound(StaticSounds::BeginningOfLine));
                    } else {
                        retained_controller_requests
                            .insert(ControllerRequest::PlaySound(StaticSounds::ServiceLogout));
                    }
                    retained_controller_requests.insert(ControllerRequest::UpdateCardList);
                }
                ControllerRequest::CloseReview => {
                    if let Ok(mut inner) = view_model.inner.lock() {
                        inner.properties.remove(&PropertieKey::ReviewScore);
                        inner
                            .volatile_properties
                            .remove(&VolatilePropertieKey::Differences);
                    }

                    retained_controller_requests
                        .insert(ControllerRequest::FetchNewCardAtThresholdOrContinue);
                }
                ControllerRequest::UpdateCardList => {
                    if self.app_data.card_list.len()==0{
                        return;
                    }

                    view_model.update_property(&PropertieKey::UserTextInput, |val| {
                        if let PropertieValue::String(ref mut user_text_input) = val {
                            user_text_input.clear();
                        }
                    });
                   

                    let mut list: HashMap<u16, u64> = HashMap::new();
                    let mut count_score_true: usize = 0;
                    let mut count_score_false: usize = 0;
                    let mut count_score_undefined: usize = 0;
                    let mut timeout_list: HashMap<u16, bool> = HashMap::new();

                    for i in 0..self.app_data.card_list.len() {
                        {
                            let last_three_indices =
                                (0..self.app_data.card_list[i].meta_data.scores.len())
                                    .rev()
                                    .take(3);
                            count_score_true += last_three_indices
                                .clone()
                                .filter_map(|x| {
                                    match self.app_data.card_list[i].meta_data.scores[x] {
                                        true => Some(true),
                                        _ => None,
                                    }
                                })
                                .count();
                            count_score_false += last_three_indices
                                .filter_map(|x| {
                                    match self.app_data.card_list[i].meta_data.scores[x] {
                                        false => Some(false),
                                        _ => None,
                                    }
                                })
                                .count();
                            count_score_undefined +=
                                if self.app_data.card_list[i].meta_data.scores.len() == 0 {
                                    1usize
                                } else {
                                    0usize
                                };
                        }
                        {
                            let time: f64 = estimate_next_session_timestamp(
                                &mut self.app_data.space_repetition_model,
                                &mut self.app_data.card_list[i].meta_data,
                            );
                            // let date = Date::new(&wasm_bindgen::JsValue::from_f64(time));
                            /*
                            super::log(&format!(
                                "Card_ID: {}\nCard_Question: {}\n{:?}\n{:?}",
                                self.card_list[i].meta_data.id,
                                self.card_list[i].display_data.get_context(),
                                time,
                                date.to_iso_string()
                            ));
                            */
                            list.insert(
                                self.app_data.card_list[i].meta_data.id,
                                time.trunc() as u64,
                            );
                        }
                        {
                            // check if there are at least 3 timestamps within the last 7 minutes
                            let seven_min_ago: f64 = Date::now() - 1000.0 * 60.0 * 7.0;
                            timeout_list.insert(
                                self.app_data.card_list[i].meta_data.id,
                                self.app_data.card_list[i]
                                    .meta_data
                                    .timestamps
                                    .iter()
                                    .filter(|&t| t >= &seven_min_ago)
                                    .count()
                                    >= 3usize,
                            );
                        }
                    }

                    let progress = count_score_true as f32
                        / ((count_score_true + count_score_false + count_score_undefined) as f32);

                    let reviewed_card_id = self.app_data.card_list[0].meta_data.id;
                    self.app_data
                        .card_list
                        .sort_unstable_by_key(|card| list.get(&card.meta_data.id));

                    // maintaining previous sort, and sort by timeout_list
                    // cards in timeout are moved to the end of the card_list.
                    self.app_data
                        .card_list
                        .sort_by_key(|card| timeout_list.get(&card.meta_data.id).map(|x| *x as u8));

                    // this prevents the same card being shown twice
                    if self.app_data.card_list.len() > 1
                        && self.app_data.card_list[0].meta_data.id == reviewed_card_id
                    {
                        self.app_data.card_list.swap(0, 1);
                    }

                    view_model
                        .insert_property(PropertieKey::Progress, PropertieValue::Float(progress));
                }
                ControllerRequest::FetchNewCardAtThresholdOrContinue => {

                    let mut progress = 0.0;

                    view_model.get_property(&PropertieKey::Progress, |val| {
                        if let PropertieValue::Float(ref val) = val {
                            progress = *val;
                        }
                    });

                    let mut fetch_new_card_at_threshold = false;
                    view_model.get_property(&PropertieKey::FetchNextCardAtThreshold, |val| {
                        if let PropertieValue::Bool(ref val) = val {
                            fetch_new_card_at_threshold = *val;
                        }
                    });
                    let mut add_new_card_threshold = 100.0;
                    view_model.get_property(&PropertieKey::AddNewCardThreshold, |val| {
                        if let PropertieValue::Float(ref val) = val {
                            add_new_card_threshold = *val;
                        }
                    });

                    if fetch_new_card_at_threshold && progress * 100.0 >= add_new_card_threshold {
                        retained_controller_requests.insert(ControllerRequest::FetchNewCard);
                    } else {
                        retained_controller_requests.insert(ControllerRequest::RefreshCard);
                    }
                }
                ControllerRequest::HideAlert => {
                    view_model.remove_property(&PropertieKey::Alert);
                }
                ControllerRequest::PlaySound(static_sound) => {
                    view_model.get_property(&PropertieKey::EnableSounds, |val| {
                        if let PropertieValue::Bool(true) = val {
                            self.app_data.static_audio.play_audio(&static_sound).ok();
                        }
                    });
                }
                ControllerRequest::PlayCardAudio => {
                    if !self.app_data.card_list[0].display_data.play_audio() {
                        retained_controller_requests.insert(ControllerRequest::PlayCardAudio);
                    }
                }
                ControllerRequest::LoadImage => {
                    if self.app_data.card_list.len()==0{
                        return;
                    }

                    let current_card = &mut self.app_data.card_list[0];

                    if current_card.display_data.has_image() {
                        if let Some(image) = current_card.display_data.get_image() {
                            if let Ok(mut inner) = view_model.inner.lock() {
                                inner.volatile_properties.insert(
                                    VolatilePropertieKey::CardImage,
                                    VolatilePropertieValue::Image(image),
                                );
                            }
                        } else {
                            retained_controller_requests.insert(ControllerRequest::LoadImage);
                        }
                    } else {
                        if let Ok(mut inner) = view_model.inner.lock() {
                            inner
                                .volatile_properties
                                .remove(&VolatilePropertieKey::CardImage);
                        }
                    }
                }
                ControllerRequest::LoadAudio => {
                    let current_card = &mut self.app_data.card_list[0];
                    if current_card.display_data.has_audio() {
                        current_card.display_data.load_audio();
                    }
                }
            }
        }

        if let Ok(mut inner) = view_model.inner.lock() {
            inner
                .controller_requests
                .extend(retained_controller_requests);
        }
    }
}

impl Default for ModelController {
    fn default() -> Self {
        Self {
            app_data: AppData::default(),
        }
    }
}
