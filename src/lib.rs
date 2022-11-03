#![warn(clippy::all, rust_2018_idioms)]

mod audio;
mod static_audio;
mod image;
mod spaced_repetition;
mod app;
pub use app::LibreLearningApp;


use wasm_bindgen::prelude::*;
 
// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

#[wasm_bindgen(module = "/defined-in-js.js")]
extern "C" {
    pub fn is_ready(); 
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
