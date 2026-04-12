pub mod ai;
mod domein;
mod iterator_ext;
pub mod model;
pub mod wasm_resultaat;

use domein::speel_zetten;
use model::{BoterKaasEieren, OngeldigeZet, Zet};
use wasm_bindgen::prelude::*;
use wasm_resultaat::WasmResultaat;

use crate::{
    ai::perfecte_zetten,
    model::{Cel, Spelstatus},
};

const DIMENSIE: usize = 3;

#[wasm_bindgen]
pub fn speel_boter_kaas_eieren(zetten: Vec<Zet>) -> WasmResultaat<BoterKaasEieren, OngeldigeZet> {
    speel_zetten(&zetten).into()
}

#[wasm_bindgen]
pub fn suggereer_zetten(zetten: Vec<Zet>) -> Vec<Zet> {
    let spel = match speel_zetten(&zetten) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    perfecte_zetten(&spel).collect()
}

pub fn speelbare_zetten(spel: &BoterKaasEieren) -> impl Iterator<Item = Zet> + '_ {
    let speler = match spel.spelstatus {
        Spelstatus::SpelBezig { speler_met_beurt } => Some(speler_met_beurt),
        _ => None,
    };

    speler.into_iter().flat_map(move |s| {
        (0..DIMENSIE).flat_map(move |y| {
            (0..DIMENSIE).filter_map(move |x| {
                if let Cel::Leeg = spel.bord[y][x] {
                    Some(Zet { x, y, speler: s })
                } else {
                    None
                }
            })
        })
    })
}
