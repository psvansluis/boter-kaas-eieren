mod domein;
pub mod model;

use domein::{nieuw_spel, speel_zet};
use model::*;
use wasm_bindgen::prelude::*;

const DIMENSIE: usize = 3;

#[wasm_bindgen]
pub fn speel_boter_kaas_eieren(zetten: Vec<Zet>) -> Result<BoterKaasEieren, OngeldigeZet> {
    zetten
        .into_iter()
        .try_fold(nieuw_spel(), |spel, zet| speel_zet(&spel, &zet))
}
