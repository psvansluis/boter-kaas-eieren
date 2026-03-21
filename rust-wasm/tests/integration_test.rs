use rstest::rstest;
use rust_wasm::*;

#[test]
fn nieuw_spel() {
    let spel = speel_boter_kaas_eieren(vec![]);
    let verwacht_bord = [[Cel::Leeg; 3]; 3];
    assert!(spel.is_ok());
    let spel = spel.unwrap();
    assert!(matches!(
        spel.spelstatus,
        Spelstatus::SpelBezig {
            speler_met_beurt: Speler::X
        }
    ));
    assert_eq!(spel.bord, verwacht_bord);
}

#[test]
fn geldige_zetten() {
    let zetten = vec![
        Zet {
            x: 0,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: 1,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 2,
            y: 0,
            speler: Speler::X,
        },
    ];
    let spel = speel_boter_kaas_eieren(zetten);
    assert!(spel.is_ok());
    let spel = spel.unwrap();
    assert!(matches!(
        spel.spelstatus,
        Spelstatus::SpelBezig {
            speler_met_beurt: Speler::O
        }
    ));
    assert_eq!(
        spel.bord,
        [
            [
                Cel::Gespeeld(Speler::X),
                Cel::Gespeeld(Speler::O),
                Cel::Gespeeld(Speler::X)
            ],
            [Cel::Leeg; 3],
            [Cel::Leeg; 3],
        ]
    );
}

#[test]
fn zet_buiten_bord() {
    let zetten = vec![Zet {
        x: 3,
        y: 0,
        speler: Speler::X,
    }];
    let spel = speel_boter_kaas_eieren(zetten);
    assert!(spel.is_err());
    assert_eq!(spel.err().unwrap(), OngeldigeZet::OngeldigeCoordinaten);
}

#[test]
fn zet_op_bezette_cel() {
    let zetten = vec![
        Zet {
            x: 0,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: 0,
            y: 0,
            speler: Speler::O,
        },
    ];
    let spel = speel_boter_kaas_eieren(zetten);
    assert!(spel.is_err());
    assert_eq!(spel.err().unwrap(), OngeldigeZet::CelBezet);
}

#[test]
fn verkeerde_speler() {
    let zetten = vec![
        Zet {
            x: 0,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: 1,
            y: 0,
            speler: Speler::X,
        },
    ];
    let spel = speel_boter_kaas_eieren(zetten);
    assert!(spel.is_err());
    assert_eq!(spel.err().unwrap(), OngeldigeZet::VerkeerdeSpeler);
}

#[rstest]
fn x_wint_verticaal(#[values(0, 1, 2)] kolom: usize) {
    let zetten = vec![
        Zet {
            x: kolom,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: (kolom + 1) % 3,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: kolom,
            y: 1,
            speler: Speler::X,
        },
        Zet {
            x: (kolom + 1) % 3,
            y: 1,
            speler: Speler::O,
        },
        Zet {
            x: kolom,
            y: 2,
            speler: Speler::X,
        },
    ];

    let spel = speel_boter_kaas_eieren(zetten);
    assert!(spel.is_ok());
    let spel = spel.unwrap();
    assert!(matches!(
        spel.spelstatus,
        Spelstatus::SpelerWint { winnaar: Speler::X }
    ));
}

#[rstest]
fn x_wint_horizontaal(#[values(0, 1, 2)] rij: usize) {
    let zetten = vec![
        Zet {
            x: 0,
            y: rij,
            speler: Speler::X,
        },
        Zet {
            x: 0,
            y: (rij + 1) % 3,
            speler: Speler::O,
        },
        Zet {
            x: 1,
            y: rij,
            speler: Speler::X,
        },
        Zet {
            x: 1,
            y: (rij + 1) % 3,
            speler: Speler::O,
        },
        Zet {
            x: 2,
            y: rij,
            speler: Speler::X,
        },
    ];

    let spel = speel_boter_kaas_eieren(zetten);
    assert!(spel.is_ok());
    let spel = spel.unwrap();
    assert!(matches!(
        spel.spelstatus,
        Spelstatus::SpelerWint { winnaar: Speler::X }
    ));
}

#[rstest]
#[case::linksboven_rechtsonder(vec![(0,0), (1,1), (2,2)])]
#[case::rechtsboven_linksonder(vec![(2,0), (1,1), (0,2)])]
fn x_wint_diagonaal(#[case] coords: Vec<(usize, usize)>) {
    let mut zetten = Vec::new();

    for (i, (x, y)) in coords.iter().enumerate() {
        // X move (diagonal)
        zetten.push(Zet {
            x: *x,
            y: *y,
            speler: Speler::X,
        });

        // O move (some safe spot that doesn't interfere)
        if i < coords.len() - 1 {
            zetten.push(Zet {
                x: (*x + 1) % 3,
                y: *y,
                speler: Speler::O,
            });
        }
    }

    let spel = speel_boter_kaas_eieren(zetten).unwrap();

    assert!(matches!(
        spel.spelstatus,
        Spelstatus::SpelerWint { winnaar: Speler::X }
    ));
}

#[test]
fn gelijkspel() {
    let zetten = vec![
        Zet {
            x: 1,
            y: 1,
            speler: Speler::X,
        },
        Zet {
            x: 0,
            y: 2,
            speler: Speler::O,
        },
        Zet {
            x: 1,
            y: 2,
            speler: Speler::X,
        },
        Zet {
            x: 1,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: 2,
            y: 2,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 1,
            speler: Speler::X,
        },
        Zet {
            x: 2,
            y: 1,
            speler: Speler::O,
        },
        Zet {
            x: 2,
            y: 0,
            speler: Speler::X,
        },
    ];
    let spel = speel_boter_kaas_eieren(zetten);
    assert!(spel.is_ok());
    let spel = spel.unwrap();
    assert!(matches!(spel.spelstatus, Spelstatus::Gelijkspel));
}

#[test]
fn zet_na_einde_spel() {
    let zetten = vec![
        Zet {
            x: 0,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: 1,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 1,
            speler: Speler::X,
        },
        Zet {
            x: 1,
            y: 1,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 2,
            speler: Speler::X,
        },
        // Deze zet komt na het einde van het spel
        Zet {
            x: 2,
            y: 2,
            speler: Speler::O,
        },
    ];
    let spel = speel_boter_kaas_eieren(zetten);
    assert!(spel.is_err());
    assert_eq!(spel.err().unwrap(), OngeldigeZet::SpelAfgerond);
}
