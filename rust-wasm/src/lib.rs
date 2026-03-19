use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoterKaasEieren {
    bord: [[char; 3]; 3],
    speler_met_beurt: char,
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {} from Rust WASM!", name)
}

#[wasm_bindgen]
pub fn speel_boter_kaas_eieren() -> JsValue {
    let spel = BoterKaasEieren {
        bord: [[' '; 3]; 3],
        speler_met_beurt: 'X',
    };
    serde_wasm_bindgen::to_value(&spel).unwrap()
}
