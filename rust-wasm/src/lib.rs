mod domein;
pub mod model;
pub mod wasm_resultaat;

use domein::{speel_zet, NIEUW_SPEL};
use model::{BoterKaasEieren, OngeldigeZet, Zet};
use wasm_bindgen::prelude::*;
use wasm_resultaat::WasmResultaat;

const DIMENSIE: usize = 3;

#[wasm_bindgen]
pub fn speel_boter_kaas_eieren(zetten: Vec<Zet>) -> WasmResultaat<BoterKaasEieren, OngeldigeZet> {
    zetten
        .into_iter()
        .try_fold(NIEUW_SPEL, |spel, zet| speel_zet(&spel, &zet))
        .into()
}
