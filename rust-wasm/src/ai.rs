use core::iter::{empty, once};

use crate::{
    domein::{speel_zet, volgende_speler},
    model::{BoterKaasEieren, Spelstatus, Zet},
    speelbare_zetten,
};

type StrategieFn = fn(&BoterKaasEieren) -> Box<dyn Iterator<Item = Zet> + '_>;

const STRATEGIEEN: &[StrategieFn] = &[
    winnende_zetten,
    voorkom_winnende_zetten,
    // |s| Box::new(vorkende_zetten(s)),
    // |s| Box::new(voorkom_vorkende_zetten(s)),
    // |s| Box::new(midden(s)),
    // |s| Box::new(tegenovergestelde_hoek(s)),
    // |s| Box::new(lege_hoek(s)),
    // |s| Box::new(lege_zijde(s)),
];

pub fn perfecte_zetten(spel: &BoterKaasEieren) -> impl Iterator<Item = Zet> + '_ {
    STRATEGIEEN
        .iter()
        .find_map(|strategie| {
            let mut zetten = strategie(spel).peekable();
            zetten.peek().is_some().then_some(zetten)
        })
        // Als er geen enkele zet mogelijk is (bijv. gelijkspel), geven we een lege iterator
        .map(|it| Box::new(it) as Box<dyn Iterator<Item = Zet>>)
        .unwrap_or_else(|| Box::new(empty()))
}

fn winnende_zetten(spel: &BoterKaasEieren) -> Box<dyn Iterator<Item = Zet> + '_> {
    Box::new(speelbare_zetten(spel).filter(|zet| {
        let spel_na_zet = speel_zet(spel, zet).expect("valide zet");
        matches!(spel_na_zet.spelstatus, Spelstatus::SpelerWint { .. })
    }))
}

fn voorkom_winnende_zetten(spel: &BoterKaasEieren) -> Box<dyn Iterator<Item = Zet> + '_> {
    Box::new(voorkom_zetten(spel, winnende_zetten))
}

// fn vorkende_zetten(_spel: &BoterKaasEieren) -> impl Iterator<Item = Zet> + '_ {
//     // if the player with the turn has moves that create a fork, return those moves
//     Vec::new()
// }

// fn voorkom_vorkende_zetten(_spel: &BoterKaasEieren) -> impl Iterator<Item = Zet> + '_ {
//     // if the opponent has moves that create a fork, return the moves that block it
//     Vec::new()
// }

// fn midden(_spel: &BoterKaasEieren) -> impl Iterator<Item = Zet> + '_ {
//     // if the middle cell is empty, return the move that plays there
//     match _spel.bord[1][1] {
//         crate::model::Cel::Leeg => vec![Zet {
//             x: 1,
//             y: 1,
//             speler: match _spel.spelstatus {
//                 Spelstatus::SpelBezig { speler_met_beurt } => speler_met_beurt,
//                 _ => unreachable!(),
//             },
//         }],
//         crate::model::Cel::Gespeeld { .. } => Vec::new(),
//     }
// }

// fn tegenovergestelde_hoek(_spel: &BoterKaasEieren) -> Vec<Zet> {
//     // if the opponent is in a corner, play the opposite corner
//     Vec::new()
// }

// fn lege_hoek(_spel: &BoterKaasEieren) -> Vec<Zet> {
//     // play an empty corner
//     Vec::new()
// }

// fn lege_zijde(_spel: &BoterKaasEieren) -> Vec<Zet> {
//     // play an empty side
//     Vec::new()
// }

// helpers
fn voorkom_zetten(
    spel: &BoterKaasEieren,
    te_voorkomen: StrategieFn,
) -> Box<dyn Iterator<Item = Zet> + '_> {
    let huidige_speler = match spel.spelstatus {
        Spelstatus::SpelBezig { speler_met_beurt } => speler_met_beurt,
        _ => return Box::new(empty()),
    };

    let nieuw_spel = BoterKaasEieren {
        bord: spel.bord,
        spelstatus: Spelstatus::SpelBezig {
            speler_met_beurt: volgende_speler(&huidige_speler),
        },
    };

    // We gebruiken std::iter::once om ownership van `nieuw_spel`
    // de iterator-keten in te trekken.
    Box::new(once(nieuw_spel).flat_map(move |s| {
        te_voorkomen(&s)
            .map(move |zet| Zet {
                x: zet.x,
                y: zet.y,
                speler: huidige_speler,
            })
            .collect::<Vec<_>>()
    }))
}
