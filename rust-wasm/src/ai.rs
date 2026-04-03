use crate::{
    domein::{speel_zet, volgende_speler},
    model::{BoterKaasEieren, Spelstatus, Zet},
    speelbare_zetten,
};

pub fn perfecte_zetten(spel: &BoterKaasEieren) -> Vec<Zet> {
    let mut speelbare_zetten = speelbare_zetten(spel);
    if speelbare_zetten.next().is_none() {
        return Vec::new();
    }
    vec![
        winnende_zetten,
        voorkom_winnende_zetten,
        vorkende_zetten,
        voorkom_vorkende_zetten,
        midden,
        tegenovergestelde_hoek,
        lege_hoek,
        lege_zijde,
    ]
    .into_iter()
    .find(|fun| !fun(spel).is_empty())
    .map(|fun| fun(spel))
    .unwrap_or(Vec::new())
}

fn winnende_zetten(spel: &BoterKaasEieren) -> Vec<Zet> {
    speelbare_zetten(spel)
        .filter(|zet| {
            let spel_na_zet = speel_zet(spel, zet).expect("valide zet");
            matches!(spel_na_zet.spelstatus, Spelstatus::SpelerWint { .. })
        })
        .collect()
}

fn voorkom_winnende_zetten(spel: &BoterKaasEieren) -> Vec<Zet> {
    voorkom_zetten(spel, winnende_zetten)
}

fn vorkende_zetten(_spel: &BoterKaasEieren) -> Vec<Zet> {
    // if the player with the turn has moves that create a fork, return those moves
    Vec::new()
}

fn voorkom_vorkende_zetten(_spel: &BoterKaasEieren) -> Vec<Zet> {
    // if the opponent has moves that create a fork, return the moves that block it
    Vec::new()
}

fn midden(_spel: &BoterKaasEieren) -> Vec<Zet> {
    // if the middle cell is empty, return the move that plays there
    match _spel.bord[1][1] {
        crate::model::Cel::Leeg => vec![Zet {
            x: 1,
            y: 1,
            speler: match _spel.spelstatus {
                Spelstatus::SpelBezig { speler_met_beurt } => speler_met_beurt,
                _ => unreachable!(),
            },
        }],
        crate::model::Cel::Gespeeld { .. } => Vec::new(),
    }
}

fn tegenovergestelde_hoek(_spel: &BoterKaasEieren) -> Vec<Zet> {
    // if the opponent is in a corner, play the opposite corner
    Vec::new()
}

fn lege_hoek(_spel: &BoterKaasEieren) -> Vec<Zet> {
    // play an empty corner
    Vec::new()
}

fn lege_zijde(_spel: &BoterKaasEieren) -> Vec<Zet> {
    // play an empty side
    Vec::new()
}

// helpers

fn voorkom_zetten(
    spel: &BoterKaasEieren,
    te_voorkomen: impl Fn(&BoterKaasEieren) -> Vec<Zet>,
) -> Vec<Zet> {
    let nieuw_spelstatus = match spel.spelstatus {
        Spelstatus::SpelBezig { speler_met_beurt } => Spelstatus::SpelBezig {
            speler_met_beurt: volgende_speler(&speler_met_beurt),
        },
        _ => unreachable!(),
    };

    let nieuw_spel = BoterKaasEieren {
        bord: spel.bord,
        spelstatus: nieuw_spelstatus,
    };

    te_voorkomen(&nieuw_spel)
        .iter()
        .map(|zet| Zet {
            x: zet.x,
            y: zet.y,
            speler: volgende_speler(&zet.speler),
        })
        .collect()
}
