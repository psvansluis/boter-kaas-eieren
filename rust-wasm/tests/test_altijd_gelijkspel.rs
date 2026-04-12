use core::panic;
use std::collections::HashMap;
use std::fmt::Debug;
use std::vec;

use rust_wasm::model::{Spelstatus, Zet};
use rust_wasm::wasm_resultaat::WasmResultaat;
use rust_wasm::{speel_boter_kaas_eieren, suggereer_zetten};

fn unwrap<T, E: Debug>(resultaat: WasmResultaat<T, E>) -> T {
    match resultaat {
        WasmResultaat::Ok(spel) => spel,
        WasmResultaat::Err(fout) => panic!("Ongeldige zet gevonden: {:?}", fout),
    }
}

/// Perfect spel bij Boter, Kaas en Eieren garandeert een gelijkspel,
/// tenzij de tegenstander een fout maakt.
/// Omdat beide spelers perfect spelen, zullen er geen winnende spellen zijn,
/// en zal het spel altijd eindigen in een gelijkspel.
/// Deze test verkent alle mogelijke paden van perfecte zetten
/// en controleert of er geen verliezende paden zijn.
#[test]
fn test_gegarandeerd_gelijkspel_bij_perfect_spel() {
    let mut start_staat = HashMap::new();
    let start_spelstatus = unwrap(speel_boter_kaas_eieren(vec![])).spelstatus;
    start_staat.insert(vec![], start_spelstatus);
    let paden = verken_perfect_spel(start_staat);

    println!("ℹ️  {} perfecte paden gecontroleerd. ", paden.len());

    let verliezende_paden: HashMap<_, _> = paden
        .iter()
        .filter(|(_, status)| matches!(status, Spelstatus::SpelerWint { .. }))
        .collect();
    assert_eq!(verliezende_paden, HashMap::new());
}

type TestMap = HashMap<Vec<Zet>, Spelstatus>;

fn verken_perfect_spel(mut map: TestMap) -> TestMap {
    let pad_om_te_verwerken = map
        .iter()
        .find(|(_, status)| matches!(status, Spelstatus::SpelBezig { .. }))
        .map(|(pad, _)| pad.clone());

    match pad_om_te_verwerken {
        Some(pad) => {
            verwerk_pad(pad, &mut map);
            verken_perfect_spel(map)
        }
        None => map,
    }
}

fn verwerk_pad(pad: Vec<Zet>, map: &mut TestMap) {
    map.remove(&pad);

    let suggesties = suggereer_zetten(pad.clone());

    if suggesties.is_empty() {
        panic!(
            "Ongeldig spelpad gevonden: {:?} heeft geen suggesties terwijl het spel bezig is!",
            pad
        );
    }

    for zet in suggesties {
        let mut nieuw_pad = pad.clone();
        nieuw_pad.push(zet);

        let nieuw_status = unwrap(speel_boter_kaas_eieren(nieuw_pad.clone())).spelstatus;
        map.insert(nieuw_pad, nieuw_status);
    }
}
