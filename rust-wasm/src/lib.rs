use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Serialize, Deserialize, Copy, Clone, Tsify, Debug)]
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

#[derive(Serialize, Deserialize, Copy, Clone, Tsify, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Zet {
    x: usize,
    y: usize,
    speler: Speler,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BoterKaasEieren {
    bord: [[Cel; 3]; 3],
    speler_met_beurt: Speler,
}

#[wasm_bindgen]
pub fn speel_boter_kaas_eieren(zetten: Vec<Zet>) -> BoterKaasEieren {
    console::log_1(&format!("Ontvangen zetten: {:?}", zetten).into());
    BoterKaasEieren {
        bord: [[Cel::Leeg; 3]; 3],
        speler_met_beurt: Speler::X,
    }
}
