use core::iter::{empty, once};

use crate::{
    domein::{speel_zet, volgende_speler},
    model::{BoterKaasEieren, Spelstatus, Zet},
    speelbare_zetten,
};

type ZetIterator<'a> = Box<dyn Iterator<Item = Zet> + 'a>;
type StrategieFn = fn(&BoterKaasEieren) -> ZetIterator<'_>;

const STRATEGIEEN: &[StrategieFn] = &[
    winnende_zetten,
    voorkom_winnende_zetten,
    vorkende_zetten,
    voorkom_vorkende_zetten,
    midden,
    lege_hoeken,
    lege_zijde,
];

/// Suggereert de beste zetten voor de huidige speler, gegeven een spel.
/// De strategieën worden in volgorde toegepast, dus als er een winnende zet is, zal die altijd als eerste worden gesuggereerd.
/// Als er geen enkele zet mogelijk is (bijv. bij een ongeldig spel of een gelijkspel), geeft deze functie een lege lijst terug.
///
/// # Voorbeeld
///
/// ```
/// use rust_wasm::{
///     model::{BoterKaasEieren, Cel, Spelstatus, Speler, Zet},
/// };
/// use rust_wasm::ai::perfecte_zetten;
/// let spel = BoterKaasEieren {
///     bord: [
///         [Cel::Leeg, Cel::Leeg, Cel::Leeg],
///         [Cel::Leeg, Cel::Leeg, Cel::Leeg],
///         [Cel::Leeg, Cel::Leeg, Cel::Leeg],
///     ],
///     spelstatus: Spelstatus::SpelBezig {
///         speler_met_beurt: Speler::X,
///     },
/// };
/// let suggesties = perfecte_zetten(&spel).collect::<Vec<_>>();
/// assert_eq!(suggesties, vec![Zet { x: 1, y: 1, speler: Speler::X }]);
/// ```
pub fn perfecte_zetten(spel: &BoterKaasEieren) -> impl Iterator<Item = Zet> + '_ {
    STRATEGIEEN
        .iter()
        .find_map(|strategie| {
            let mut zetten = strategie(spel).peekable();
            zetten.peek()?;
            Some(Box::new(zetten) as ZetIterator<'_>)
        })
        // Als er geen enkele zet mogelijk is (bijv. gelijkspel), geven we een lege iterator
        .unwrap_or_else(|| Box::new(empty()))
}

fn winnende_zetten(spel: &BoterKaasEieren) -> ZetIterator<'_> {
    filter_op_resultaat(spel, |s| {
        matches!(s.spelstatus, Spelstatus::SpelerWint { .. })
    })
}

fn voorkom_winnende_zetten(spel: &BoterKaasEieren) -> ZetIterator<'_> {
    voorkom_zetten(spel, winnende_zetten)
}

fn vorkende_zetten(spel: &BoterKaasEieren) -> ZetIterator<'_> {
    filter_op_resultaat(spel, |s| voorkom_winnende_zetten(s).nth(1).is_some())
}

fn voorkom_vorkende_zetten(spel: &BoterKaasEieren) -> ZetIterator<'_> {
    voorkom_zetten(spel, vorkende_zetten)
}

fn midden(spel: &BoterKaasEieren) -> ZetIterator<'_> {
    filter_coordinaten(spel, &[(1, 1)])
}

fn lege_hoeken(spel: &BoterKaasEieren) -> ZetIterator<'_> {
    filter_coordinaten(spel, &[(0, 2), (0, 0), (2, 0), (2, 2)])
}

fn lege_zijde(spel: &BoterKaasEieren) -> ZetIterator<'_> {
    filter_coordinaten(spel, &[(0, 1), (1, 0), (1, 2), (2, 1)])
}

// helpers
fn voorkom_zetten(spel: &BoterKaasEieren, te_voorkomen: StrategieFn) -> ZetIterator<'_> {
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

fn filter_op_resultaat<'a>(
    spel: &'a BoterKaasEieren,
    predicaat: impl Fn(&BoterKaasEieren) -> bool + 'a,
) -> ZetIterator<'a> {
    Box::new(speelbare_zetten(spel).filter(move |zet| {
        let spel_na_zet = speel_zet(spel, zet).expect("valide zet");
        predicaat(&spel_na_zet)
    }))
}

fn filter_zetten<'a>(
    spel: &'a BoterKaasEieren,
    predicaat: impl Fn(&Zet) -> bool + 'a,
) -> ZetIterator<'a> {
    Box::new(speelbare_zetten(spel).filter(predicaat))
}

fn filter_coordinaten<'a>(
    spel: &'a BoterKaasEieren,
    coordinaten: &'a [(usize, usize)],
) -> ZetIterator<'a> {
    filter_zetten(spel, |zet| {
        coordinaten.iter().any(|(x, y)| *x == zet.x && *y == zet.y)
    })
}
