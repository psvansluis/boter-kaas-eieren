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
    let mut spel = nieuw_spel();
    for zet in zetten {
        speel_zet(&mut spel, zet);
    }
    spel
}

fn nieuw_spel() -> BoterKaasEieren {
    BoterKaasEieren {
        bord: [[Cel::Leeg; 3]; 3],
        speler_met_beurt: Speler::X,
    }
}

fn speel_zet(spel: &mut BoterKaasEieren, zet: Zet) {
    spel.bord[zet.y][zet.x] = Cel::Gespeeld(zet.speler);
    spel.speler_met_beurt = match zet.speler {
        Speler::X => Speler::O,
        Speler::O => Speler::X,
    };
}
