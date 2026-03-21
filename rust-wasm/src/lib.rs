use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

const DIMENSIE: usize = 3;
pub type Bord = [[Cel; DIMENSIE]; DIMENSIE];

#[derive(Serialize, Deserialize, Copy, Clone, Tsify, Debug, Eq, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Speler {
    X,
    O,
}

#[derive(Serialize, Deserialize, Copy, Clone, Tsify, Debug, Eq, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Cel {
    Leeg,
    Gespeeld(Speler),
}

#[derive(Serialize, Deserialize, Tsify, Debug, Eq, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Spelstatus {
    SpelerWint { winnaar: Speler },
    Gelijkspel,
    SpelBezig { speler_met_beurt: Speler },
}

#[derive(Serialize, Deserialize, Copy, Clone, Tsify, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Zet {
    pub x: usize,
    pub y: usize,
    pub speler: Speler,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BoterKaasEieren {
    pub bord: Bord,
    pub spelstatus: Spelstatus,
}

#[derive(Serialize, Deserialize, Tsify, Debug, Eq, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum OngeldigeZet {
    OngeldigeCoordinaten,
    CelBezet,
    VerkeerdeSpeler,
    SpelAfgerond,
}

#[wasm_bindgen]
pub fn speel_boter_kaas_eieren(zetten: Vec<Zet>) -> Result<BoterKaasEieren, OngeldigeZet> {
    zetten
        .into_iter()
        .try_fold(nieuw_spel(), |spel, zet| speel_zet(&spel, &zet))
}

fn nieuw_spel() -> BoterKaasEieren {
    BoterKaasEieren {
        bord: [[Cel::Leeg; DIMENSIE]; DIMENSIE],
        spelstatus: Spelstatus::SpelBezig {
            speler_met_beurt: Speler::X,
        },
    }
}

fn speel_zet(spel: &BoterKaasEieren, zet: &Zet) -> Result<BoterKaasEieren, OngeldigeZet> {
    if zet.x >= DIMENSIE || zet.y >= DIMENSIE {
        return Err(OngeldigeZet::OngeldigeCoordinaten);
    }
    if let Cel::Gespeeld(_) = spel.bord[zet.y][zet.x] {
        return Err(OngeldigeZet::CelBezet);
    }
    match spel.spelstatus {
        Spelstatus::SpelerWint { .. } | Spelstatus::Gelijkspel => {
            return Err(OngeldigeZet::SpelAfgerond);
        }
        Spelstatus::SpelBezig { speler_met_beurt } => {
            if zet.speler != speler_met_beurt {
                return Err(OngeldigeZet::VerkeerdeSpeler);
            }
        }
    }
    let mut nieuw_bord = spel.bord;
    nieuw_bord[zet.y][zet.x] = Cel::Gespeeld(zet.speler);
    let volgende_speler = match zet.speler {
        Speler::X => Speler::O,
        Speler::O => Speler::X,
    };

    let nieuw_spelstatus = bepaal_spelstatus(&nieuw_bord, volgende_speler);

    Ok(BoterKaasEieren {
        bord: nieuw_bord,
        spelstatus: nieuw_spelstatus,
    })
}

fn bepaal_spelstatus(bord: &Bord, speler_met_beurt: Speler) -> Spelstatus {
    match check_winnaar(&bord) {
        Some(winnaar) => Spelstatus::SpelerWint { winnaar },
        _ if is_bord_vol(&bord) => Spelstatus::Gelijkspel,
        _ => Spelstatus::SpelBezig { speler_met_beurt },
    }
}

fn is_bord_vol(bord: &Bord) -> bool {
    bord.iter()
        .all(|rij| rij.iter().all(|cel| matches!(cel, Cel::Gespeeld(_))))
}

fn check_winnaar(bord: &Bord) -> Option<Speler> {
    // Check rijen
    for y in 0..DIMENSIE {
        if let Cel::Gespeeld(speler) = bord[y][0] {
            if (1..DIMENSIE).all(|x| bord[y][x] == Cel::Gespeeld(speler)) {
                return Some(speler);
            }
        }
    }

    // Check kolommen
    for x in 0..DIMENSIE {
        if let Cel::Gespeeld(speler) = bord[0][x] {
            if (1..DIMENSIE).all(|y| bord[y][x] == Cel::Gespeeld(speler)) {
                return Some(speler);
            }
        }
    }

    // Check diagonalen
    if let Cel::Gespeeld(speler) = bord[0][0] {
        if (1..DIMENSIE).all(|i| bord[i][i] == Cel::Gespeeld(speler)) {
            return Some(speler);
        }
    }
    if let Cel::Gespeeld(speler) = bord[0][DIMENSIE - 1] {
        if (1..DIMENSIE).all(|i| bord[i][DIMENSIE - 1 - i] == Cel::Gespeeld(speler)) {
            return Some(speler);
        }
    }

    None
}
