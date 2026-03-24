use rstest::rstest;
use rust_wasm::model::{BoterKaasEieren, Cel, OngeldigeZet, Speler, Spelstatus, Zet};
use rust_wasm::wasm_resultaat::WasmResultaat;
use rust_wasm::*;

fn assert_ok_en(
    spel: WasmResultaat<BoterKaasEieren, OngeldigeZet>,
    assertie: &dyn Fn(&BoterKaasEieren) -> (),
) {
    assert!(matches!(
        spel,
        rust_wasm::wasm_resultaat::WasmResultaat::Ok(_)
    ));
    let spel = match spel {
        WasmResultaat::Ok(spel) => spel,
        _ => unreachable!(),
    };
    assertie(&spel);
}

fn assert_err_en(spel: WasmResultaat<BoterKaasEieren, OngeldigeZet>, verwachte_fout: OngeldigeZet) {
    assert!(matches!(
        spel,
        rust_wasm::wasm_resultaat::WasmResultaat::Err(_)
    ));
    let fout = match spel {
        WasmResultaat::Err(fout) => fout,
        _ => unreachable!(),
    };
    assert_eq!(fout, verwachte_fout);
}

#[test]
fn nieuw_spel() {
    let spel = speel_boter_kaas_eieren(vec![]);
    let verwacht_bord = [[Cel::Leeg; 3]; 3];
    let verwachte_spelstatus = Spelstatus::SpelBezig {
        speler_met_beurt: Speler::X,
    };
    let assertie = |spel: &BoterKaasEieren| {
        assert_eq!(spel.bord, verwacht_bord);
        assert_eq!(spel.spelstatus, verwachte_spelstatus);
    };
    assert_ok_en(spel, &assertie);
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
    let verwacht_bord = [
        [
            Cel::Gespeeld { door: Speler::X },
            Cel::Gespeeld { door: Speler::X },
            Cel::Gespeeld { door: Speler::X },
        ],
        [Cel::Leeg; 3],
        [Cel::Leeg; 3],
    ];
    let assertie = |spel: &BoterKaasEieren| {
        assert_eq!(spel.bord, verwacht_bord);
        assert_eq!(
            spel.spelstatus,
            Spelstatus::SpelBezig {
                speler_met_beurt: Speler::O,
            }
        );
    };
    assert_ok_en(spel, &assertie);
}

#[test]
fn zet_buiten_bord() {
    let zetten = vec![Zet {
        x: 3,
        y: 0,
        speler: Speler::X,
    }];
    let spel = speel_boter_kaas_eieren(zetten);
    let verwachte_fout = OngeldigeZet::OngeldigeCoordinaten;
    assert_err_en(spel, verwachte_fout);
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
    assert_err_en(spel, OngeldigeZet::CelBezet);
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
    assert_err_en(spel, OngeldigeZet::VerkeerdeSpeler);
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
    let assertie = |spel: &BoterKaasEieren| {
        assert!(matches!(
            spel.spelstatus,
            Spelstatus::SpelerWint { winnaar: Speler::X }
        ));
    };
    assert_ok_en(spel, &assertie);
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
    let assertie = |spel: &BoterKaasEieren| {
        assert!(matches!(
            spel.spelstatus,
            Spelstatus::SpelerWint { winnaar: Speler::X }
        ));
    };
    assert_ok_en(spel, &assertie);
}

#[rstest]
#[case::linksboven_rechtsonder(vec![(0,0), (1,1), (2,2)])]
#[case::rechtsboven_linksonder(vec![(2,0), (1,1), (0,2)])]
fn x_wint_diagonaal(#[case] coords: Vec<(usize, usize)>) {
    let mut zetten = Vec::new();

    for (i, (x, y)) in coords.iter().enumerate() {
        // X plaatst zet diagonaal
        zetten.push(Zet {
            x: *x,
            y: *y,
            speler: Speler::X,
        });

        // O plaatst zet (een veilige plek die niet in de weg staat)
        if i < coords.len() - 1 {
            zetten.push(Zet {
                x: (*x + 1) % 3,
                y: *y,
                speler: Speler::O,
            });
        }
    }

    let spel = speel_boter_kaas_eieren(zetten);

    assert_ok_en(spel, &|spel: &BoterKaasEieren| {
        assert!(matches!(
            spel.spelstatus,
            Spelstatus::SpelerWint { winnaar: Speler::X }
        ));
    });
}

#[test]
fn o_wint() {
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
            x: 2,
            y: 2,
            speler: Speler::X,
        },
        Zet {
            x: 1,
            y: 2,
            speler: Speler::O,
        },
    ];

    let spel = speel_boter_kaas_eieren(zetten);
    assert_ok_en(spel, &|spel: &BoterKaasEieren| {
        assert!(matches!(
            spel.spelstatus,
            Spelstatus::SpelerWint { winnaar: Speler::O }
        ));
    });
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
    let assertie = |spel: &BoterKaasEieren| {
        assert_eq!(spel.spelstatus, Spelstatus::Gelijkspel);
    };
    assert_ok_en(spel, &assertie);
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
    assert_err_en(spel, OngeldigeZet::SpelAfgerond);
}
