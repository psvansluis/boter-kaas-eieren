pub mod ai;
mod domein;
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
pub fn suggereer_zet(zetten: Vec<Zet>) -> Option<Zet> {
    let spel = speel_zetten(&zetten).ok()?;

    // for now take the first move from perfecte_zetten,
    // but in the future we should pick one at random
    perfecte_zetten(&spel).into_iter().next()
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
