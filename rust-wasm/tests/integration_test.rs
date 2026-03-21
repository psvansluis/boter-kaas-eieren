use rust_wasm::*;

#[test]
fn nieuw_spel() {
    let spel = speel_boter_kaas_eieren(vec![]);
    let verwachte_speler_met_beurt = Speler::X;
    let verwacht_bord = [[Cel::Leeg; 3]; 3];
    assert!(spel.is_ok());
    let spel = spel.unwrap();
    if let Spelstatus::SpelBezig { speler_met_beurt } = spel.spelstatus {
        assert_eq!(speler_met_beurt, verwachte_speler_met_beurt);
    } else {
        panic!("Verwachte spelstatus SpelBezig");
    }
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
    if let Spelstatus::SpelBezig { speler_met_beurt } = spel.spelstatus {
        assert_eq!(speler_met_beurt, Speler::O);
    } else {
        panic!("Verwachte spelstatus SpelBezig");
    }
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
