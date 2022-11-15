// Everything the UI need to know. No more, no less.

use crate::app_controller::model_controller::data_model::download::DownloadState;
use crate::app_controller::model_controller::data_model::static_audio::StaticSounds;
use difference::Difference;
use egui_extras::RetainedImage;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum ControllerRequest {
    ResetApp,
    SaveCheckpoint,
    LoadCheckpoint,
    DeleteCheckpoint,
    UpdateRequestConfig,
    UpdateAIRequestConfig,
    TestCustomServerConnection(bool),
    TestAIServerConnection(bool),
    FetchNewCard,
    CheckReview,
    HideAlert,
    PlaySound(StaticSounds),
    PlayCardAudio,
    LoadImage,
    LoadAudio,
    CloseReview,
    UpdateCardList,
    FetchNewCardAtThresholdOrContinue,
    RefreshCard,
    RefreshRequestConfig,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DisplayKind {
    AppDisplay,
    APISettingsDisplay,
    SaveLoadSettingsDisplay,
    OptionsSettingsDisplay,
}

impl Default for DisplayKind {
    fn default() -> Self {
        DisplayKind::AppDisplay
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Eq, Hash)]
pub enum PropertieKey {
    AutoPlayAudio,
    EnableSounds,
    FetchNextCardAtThreshold,
    ConnectToCustomServer,
    FetchDalleGeneratedImages,
    EnableGPT3CardGeneration,
    AddNewCardThreshold,
    SpellingCorrectionThreshold,
    IgnoreSentencePunctuationSymbols,
    MatchASCII,
    MatchCase,
    CustomServerEndpoint,
    CustomServerUsername,
    CustomServerPassword,
    CustomServerConnectionStatus,
    Progress,
    UserTextInput,
    UserTextInputHint,
    Checkpoints,
    SelectedCheckpoint,
    AIServerEndpoint,
    AIServerUsername,
    AIServerPassword,
    AIServerConnectionStatus,
    CardQuestion,
    CardContext,
    CardHasAudio,
    Alert,
    ReviewScore,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum PropertieValue {
    Bool(bool),
    Float(f32),
    Usize(usize),
    String(String),
    VecString(Vec<String>),
    DownloadState(DownloadState),
}

#[derive(PartialEq, Eq, Hash)]
pub enum VolatilePropertieKey {
    CardImage,
    Differences,
}

pub enum VolatilePropertieValue {
    Image(Arc<Mutex<RetainedImage>>),
    Differences(Vec<Difference>),
}

#[derive(serde::Deserialize, serde::Serialize, derivative::Derivative)]
#[derivative(Debug)]
pub struct InnerViewModel {
    #[serde(skip)]
    pub display_kind: DisplayKind,

    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub controller_requests: HashSet<ControllerRequest>,

    #[derivative(Debug = "ignore")]
    pub properties: HashMap<PropertieKey, PropertieValue>,

    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub volatile_properties: HashMap<VolatilePropertieKey, VolatilePropertieValue>,
}

impl Default for InnerViewModel {
    fn default() -> Self {
        Self {
            display_kind: DisplayKind::default(),
            controller_requests: HashSet::new(),
            properties: HashMap::from([
                (PropertieKey::AutoPlayAudio, PropertieValue::Bool(false)),
                (PropertieKey::EnableSounds, PropertieValue::Bool(true)),
                (
                    PropertieKey::FetchNextCardAtThreshold,
                    PropertieValue::Bool(true),
                ),
                (
                    PropertieKey::ConnectToCustomServer,
                    PropertieValue::Bool(true),
                ),
                (
                    PropertieKey::FetchDalleGeneratedImages,
                    PropertieValue::Bool(false),
                ),
                (
                    PropertieKey::EnableGPT3CardGeneration,
                    PropertieValue::Bool(false),
                ),
                (
                    PropertieKey::AddNewCardThreshold,
                    PropertieValue::Float(66.6),
                ),
                (
                    PropertieKey::SpellingCorrectionThreshold,
                    PropertieValue::Usize(1),
                ),
                (
                    PropertieKey::IgnoreSentencePunctuationSymbols,
                    PropertieValue::Bool(true),
                ),
                (PropertieKey::MatchASCII, PropertieValue::Bool(false)),
                (PropertieKey::MatchCase, PropertieValue::Bool(false)),
                (
                    PropertieKey::CustomServerEndpoint,
                    PropertieValue::String("".to_string()),
                ),
                (
                    PropertieKey::CustomServerUsername,
                    PropertieValue::String("".to_string()),
                ),
                (
                    PropertieKey::CustomServerPassword,
                    PropertieValue::String("".to_string()),
                ),
                (PropertieKey::Progress, PropertieValue::Float(0.0)),
                (
                    PropertieKey::UserTextInput,
                    PropertieValue::String("".to_string()),
                ),
                (PropertieKey::UserTextInputHint,
                PropertieValue::String("".to_string()),
                ),
                (
                    PropertieKey::Checkpoints,
                    PropertieValue::VecString(Vec::new()),
                ),
                /*(
                    PropertieKey::SelectedCheckpoint,
                    PropertieValue::String("http://127.0.0.1:8081/80331a/ntnu".to_string()),
                ),*/
                (
                    PropertieKey::CustomServerConnectionStatus,
                    PropertieValue::DownloadState(DownloadState::None),
                ),
                (
                    PropertieKey::AIServerEndpoint,
                    PropertieValue::String("".to_string()),
                ),
                (
                    PropertieKey::AIServerUsername,
                    PropertieValue::String("".to_string()),
                ),
                (
                    PropertieKey::AIServerPassword,
                    PropertieValue::String("".to_string()),
                ),
                (
                    PropertieKey::AIServerConnectionStatus,
                    PropertieValue::DownloadState(DownloadState::None),
                ),
                (
                    PropertieKey::CardQuestion,
                    PropertieValue::String("For when you just want to learn and memorize.".to_string()),
                ),
                (
                    PropertieKey::CardContext,
                    PropertieValue::String("Learn what you need, when you need it.".to_string()),
                ),
                (
                    PropertieKey::CardHasAudio,
                    PropertieValue::Bool(false),
                ),
                /*
                (
                    PropertieKey::Alert,
                    PropertieValue::String("This is a test!".to_string()),
                ),*/
            ]),
            volatile_properties: HashMap::new(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ViewModel {
    pub inner: Arc<Mutex<InnerViewModel>>,
}
impl ViewModel {
    pub fn get_property<F>(&self, key: &PropertieKey, mut f: F)
    where
        F: FnMut(&PropertieValue),
    {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(ref val) = inner.properties.get(key) {
                f(val)
            }
        }
    }

    pub fn update_property<F>(&self, key: &PropertieKey, mut f: F)
    where
        F: FnMut(&mut PropertieValue),
    {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(ref mut val) = inner.properties.get_mut(key) {
                f(val)
            }
        }
    }

    pub fn update_volatile_property<F>(&self, key: &VolatilePropertieKey, mut f: F)
    where
        F: FnMut(&mut VolatilePropertieValue),
    {
        if let Ok(mut inner) = self.inner.lock() {
            if let Some(ref mut val) = inner.volatile_properties.get_mut(key) {
                f(val)
            }
        }
    }

    pub fn insert_property(&self, key: PropertieKey, new: PropertieValue) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.properties.insert(key, new);
        }
    }
    pub fn remove_property(&self, key: &PropertieKey) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.properties.remove(key);
        }
    }

    /*
    pub fn update_property_bool<F>(&self, key: &PropertieKey, mut f: F)  where
         F: FnMut(&mut bool) {
        self.update_property(key, |val| {
            if let PropertieValue::Bool(ref mut v) = val {
                f(v)
            }
        })
    }*/
}

impl Default for ViewModel {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(InnerViewModel::default())),
        }
    }
}
