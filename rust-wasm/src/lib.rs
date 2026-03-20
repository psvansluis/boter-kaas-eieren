use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Copy, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Speler {
    X,
    O,
}

#[derive(Serialize, Deserialize, Copy, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Cel {
    Leeg,
    Gespeeld(Speler),
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BoterKaasEieren {
    bord: [[Cel; 3]; 3],
    speler_met_beurt: Speler,
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {} from Rust WASM!", name)
}

#[wasm_bindgen]
pub fn speel_boter_kaas_eieren() -> BoterKaasEieren {
    BoterKaasEieren {
        bord: [[Cel::Leeg; 3]; 3],
        speler_met_beurt: Speler::X,
    }
}
