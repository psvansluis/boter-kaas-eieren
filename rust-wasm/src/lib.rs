mod domein;
pub mod model;
pub mod wasm_resultaat;

use domein::{nieuw_spel, speel_zet};
use model::*;
use wasm_bindgen::prelude::*;
use wasm_resultaat::WasmResultaat;

const DIMENSIE: usize = 3;

#[wasm_bindgen]
pub fn speel_boter_kaas_eieren(zetten: Vec<Zet>) -> WasmResultaat<BoterKaasEieren, OngeldigeZet> {
    zetten
        .into_iter()
        .try_fold(nieuw_spel(), |spel, zet| speel_zet(&spel, &zet))
        .into()
}
