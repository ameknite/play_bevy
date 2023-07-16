#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use bevy::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}