use rust_wasm::{
    model::{Speler, Zet},
    suggereer_zet,
};

#[test]
fn onspeelbaar_spel_heeft_geen_suggesties() {
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
    ];

    let suggestie = suggereer_zet(zetten);
    assert!(suggestie.is_none());
}

#[test]
fn winnende_zet_wordt_gesuggereerd() {
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
    ];

    let suggestie = suggereer_zet(zetten);
    assert_eq!(
        suggestie,
        Some(Zet {
            x: 0,
            y: 2,
            speler: Speler::X
        })
    );
}

#[test]
fn blokkerende_zet_wordt_gesuggereerd() {
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
    ];

    let suggestie = suggereer_zet(zetten);
    assert_eq!(
        suggestie,
        Some(Zet {
            x: 0,
            y: 2,
            speler: Speler::O
        })
    );
}
