use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::DIMENSIE;

pub type Bord = [[Cel; DIMENSIE]; DIMENSIE];

#[derive(Serialize, Deserialize, Copy, Clone, Tsify, Debug, Eq, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type", content = "data")]
pub enum Speler {
    X,
    O,
}

#[derive(Serialize, Deserialize, Copy, Clone, Tsify, Debug, Eq, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type", content = "data")]
pub enum Cel {
    Leeg,
    Gespeeld { door: Speler },
}

#[derive(Serialize, Deserialize, Tsify, Debug, Eq, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type", content = "data")]
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
#[serde(tag = "type", content = "data")]
pub enum OngeldigeZet {
    OngeldigeCoordinaten,
    CelBezet,
    VerkeerdeSpeler,
    SpelAfgerond,
}
