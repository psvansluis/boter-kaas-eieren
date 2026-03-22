use crate::{
    model::{Bord, BoterKaasEieren, Cel, OngeldigeZet, Speler, Spelstatus, Zet},
    DIMENSIE,
};

pub fn nieuw_spel() -> BoterKaasEieren {
    BoterKaasEieren {
        bord: [[Cel::Leeg; DIMENSIE]; DIMENSIE],
        spelstatus: Spelstatus::SpelBezig {
            speler_met_beurt: Speler::X,
        },
    }
}

pub fn speel_zet(spel: &BoterKaasEieren, zet: &Zet) -> Result<BoterKaasEieren, OngeldigeZet> {
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

pub fn winnende_lijnen(dimensie: usize) -> Vec<Vec<(usize, usize)>> {
    let mut lines = Vec::new();

    if dimensie == 0 {
        return lines;
    }

    // Rijen
    for y in 0..dimensie {
        lines.push((0..dimensie).map(|x| (x, y)).collect());
    }

    // Kolommen
    for x in 0..dimensie {
        lines.push((0..dimensie).map(|y| (x, y)).collect());
    }

    // Diagonalen
    lines.push((0..dimensie).map(|i| (i, i)).collect());
    lines.push((0..dimensie).map(|i| (dimensie - 1 - i, i)).collect());

    lines
}

fn winnaar_op_lijn(bord: &Bord, coords: &[(usize, usize)]) -> Option<Speler> {
    let (x0, y0) = coords[0];
    if let Cel::Gespeeld(speler) = bord[y0][x0] {
        if coords
            .iter()
            .skip(1)
            .all(|&(x, y)| bord[y][x] == Cel::Gespeeld(speler))
        {
            return Some(speler);
        }
    }
    None
}

fn check_winnaar(bord: &Bord) -> Option<Speler> {
    winnende_lijnen(DIMENSIE)
        .iter()
        .filter_map(|line| winnaar_op_lijn(bord, line))
        .next()
}
