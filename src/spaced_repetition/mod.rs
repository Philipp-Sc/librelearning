use super::app::CardMetaData;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/defined-in-js.js")]
extern "C" {

    pub type SpacedRepetition;

    #[wasm_bindgen(constructor)]
    fn new() -> SpacedRepetition;

    #[wasm_bindgen(method)]
    fn calculateEBISU(
        this: &SpacedRepetition,
        card_id: u16,
        timestamps: Vec<f64>,
        scores: Vec<js_sys::Boolean>,
    ) -> f64;
}

impl Default for SpacedRepetition {
    fn default() -> Self {
        SpacedRepetition::new()
    }
}

pub fn estimate_next_session_timestamp(
    my_class: &mut SpacedRepetition,
    meta_data: &mut CardMetaData,
) -> f64 {
    my_class.calculateEBISU(
        meta_data.id,
        meta_data.timestamps.clone(),
        meta_data
            .scores
            .clone()
            .into_iter()
            .map(|x| js_sys::Boolean::from(x))
            .collect(),
    )
}
