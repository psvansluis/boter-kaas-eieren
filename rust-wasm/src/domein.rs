use core::iter::once;

use crate::{
    model::{Bord, BoterKaasEieren, Cel, OngeldigeZet, Speler, Spelstatus, Zet},
    DIMENSIE,
};

pub const NIEUW_SPEL: BoterKaasEieren = BoterKaasEieren {
    bord: [[Cel::Leeg; DIMENSIE]; DIMENSIE],
    spelstatus: Spelstatus::SpelBezig {
        speler_met_beurt: Speler::X,
    },
};

pub fn speel_zet(spel: &BoterKaasEieren, zet: &Zet) -> Result<BoterKaasEieren, OngeldigeZet> {
    valideer_zet(spel, zet)?;

    let mut nieuw_bord = spel.bord;
    nieuw_bord[zet.y][zet.x] = Cel::Gespeeld { door: zet.speler };
    let nieuw_spelstatus = bepaal_spelstatus(&nieuw_bord, volgende_speler(&zet.speler));

    Ok(BoterKaasEieren {
        bord: nieuw_bord,
        spelstatus: nieuw_spelstatus,
    })
}

fn valideer_zet(spel: &BoterKaasEieren, zet: &Zet) -> Result<(), OngeldigeZet> {
    if zet.x >= DIMENSIE || zet.y >= DIMENSIE {
        return Err(OngeldigeZet::OngeldigeCoordinaten);
    }
    if let Cel::Gespeeld { .. } = spel.bord[zet.y][zet.x] {
        return Err(OngeldigeZet::CelBezet);
    }
    if let Spelstatus::SpelBezig { speler_met_beurt } = spel.spelstatus {
        if zet.speler != speler_met_beurt {
            return Err(OngeldigeZet::VerkeerdeSpeler);
        }
    } else {
        return Err(OngeldigeZet::SpelAfgerond);
    }
    Ok(())
}

const fn volgende_speler(speler: &Speler) -> Speler {
    match speler {
        Speler::X => Speler::O,
        Speler::O => Speler::X,
    }
}

fn bepaal_spelstatus(bord: &Bord, speler_met_beurt: Speler) -> Spelstatus {
    match check_winnaar(bord) {
        Some(winnaar) => Spelstatus::SpelerWint { winnaar },
        _ if is_bord_vol(bord) => Spelstatus::Gelijkspel,
        _ => Spelstatus::SpelBezig { speler_met_beurt },
    }
}

fn is_bord_vol(bord: &Bord) -> bool {
    bord.iter()
        .all(|rij| rij.iter().all(|cel| matches!(cel, Cel::Gespeeld { .. })))
}

type Coordinaat = (usize, usize);
type Lijn = Box<dyn Iterator<Item = Coordinaat>>;
type BoxedIterator<T> = Box<dyn Iterator<Item = T>>;

pub fn winnende_lijnen(dimensie: usize) -> BoxedIterator<Lijn> {
    let rijen = (0..dimensie).map(move |y| Box::new((0..dimensie).map(move |x| (x, y))) as Lijn);
    let kolommen = (0..dimensie).map(move |x| Box::new((0..dimensie).map(move |y| (x, y))) as Lijn);
    let hoofddiagonaal = Box::new((0..dimensie).map(|i| (i, i))) as Lijn;
    let antidiagonaal = Box::new((0..dimensie).map(move |i| (i, dimensie - 1 - i))) as Lijn;

    Box::new(
        rijen
            .chain(kolommen)
            .chain(once(hoofddiagonaal))
            .chain(once(antidiagonaal)),
    )
}

fn winnaar_op_lijn(bord: &Bord, mut coordinaten: BoxedIterator<Coordinaat>) -> Option<Speler> {
    let (x0, y0) = coordinaten.next()?;
    let Cel::Gespeeld { door } = bord[y0][x0] else {
        return None;
    };
    coordinaten
        .all(|(x, y)| bord[y][x] == Cel::Gespeeld { door })
        .then_some(door)
}

fn check_winnaar(bord: &Bord) -> Option<Speler> {
    winnende_lijnen(DIMENSIE).find_map(|lijn| winnaar_op_lijn(bord, lijn))
}
