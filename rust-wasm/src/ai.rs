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
    vorkende_zetten,
    voorkom_vorkende_zetten,
    midden,
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
        .map(|it| Box::new(it) as Box<dyn Iterator<Item = Zet>>)
        // Als er geen enkele zet mogelijk is (bijv. gelijkspel), geven we een lege iterator
        .unwrap_or_else(|| Box::new(empty()))
}

fn winnende_zetten(spel: &BoterKaasEieren) -> Box<dyn Iterator<Item = Zet> + '_> {
    filter_op_resultaat(spel, |s| {
        matches!(s.spelstatus, Spelstatus::SpelerWint { .. })
    })
}

fn voorkom_winnende_zetten(spel: &BoterKaasEieren) -> Box<dyn Iterator<Item = Zet> + '_> {
    Box::new(voorkom_zetten(spel, winnende_zetten))
}

fn vorkende_zetten(spel: &BoterKaasEieren) -> Box<dyn Iterator<Item = Zet> + '_> {
    filter_op_resultaat(spel, |s| voorkom_winnende_zetten(s).nth(1).is_some())
}

fn voorkom_vorkende_zetten(spel: &BoterKaasEieren) -> Box<dyn Iterator<Item = Zet> + '_> {
    Box::new(voorkom_zetten(spel, vorkende_zetten))
}

fn midden(spel: &BoterKaasEieren) -> Box<dyn Iterator<Item = Zet> + '_> {
    Box::new(speelbare_zetten(spel).filter(|zet| zet.x == 1 && zet.y == 1))
}

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

fn filter_op_resultaat<P>(
    spel: &BoterKaasEieren,
    predicaat: P,
) -> Box<dyn Iterator<Item = Zet> + '_>
where
    P: Fn(&BoterKaasEieren) -> bool + 'static,
{
    Box::new(speelbare_zetten(spel).filter(move |zet| {
        let spel_na_zet = speel_zet(spel, zet).expect("valide zet");
        predicaat(&spel_na_zet)
    }))
}
