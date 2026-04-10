use rust_wasm::{
    model::{Speler, Zet},
    suggereer_zetten,
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

    let suggestie = suggereer_zetten(zetten);
    assert_eq!(suggestie.len(), 0);
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

    let suggestie = suggereer_zetten(zetten);
    assert_eq!(
        suggestie,
        vec![Zet {
            x: 0,
            y: 2,
            speler: Speler::X
        }]
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

    let suggestie = suggereer_zetten(zetten);
    assert_eq!(
        suggestie,
        vec![Zet {
            x: 0,
            y: 2,
            speler: Speler::O
        }]
    );
}

#[test]
fn vorkende_zet_wordt_gesuggereerd() {
    let zetten = vec![
        Zet {
            x: 0,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: 2,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 2,
            speler: Speler::X,
        },
        Zet {
            x: 0,
            y: 1,
            speler: Speler::O,
        },
    ];

    let suggestie = suggereer_zetten(zetten);
    let verwachte_suggesties = vec![Zet {
        x: 2,
        y: 2,
        speler: Speler::X,
    }];
    assert_eq!(suggestie, verwachte_suggesties);
}

#[test]
fn voorkom_vorkende_zet_wordt_gesuggereerd() {
    let zetten = vec![
        Zet {
            x: 0,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: 0,
            y: 1,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 2,
            speler: Speler::X,
        },
    ];

    let suggestie = suggereer_zetten(zetten);
    let verwachte_suggesties = vec![
        Zet {
            x: 2,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 1,
            y: 1,
            speler: Speler::O,
        },
        Zet {
            x: 2,
            y: 2,
            speler: Speler::O,
        },
    ];
    assert_eq!(suggestie, verwachte_suggesties);
}

#[test]
fn midden_zet_wordt_gesuggereerd() {
    let zetten = vec![];
    let suggestie = suggereer_zetten(zetten);
    let verwachte_suggesties = vec![Zet {
        x: 1,
        y: 1,
        speler: Speler::X,
    }];
    assert_eq!(suggestie, verwachte_suggesties);
}

#[test]
fn hoek_zet_wordt_gesuggereerd() {
    let zetten = vec![Zet {
        x: 1,
        y: 1,
        speler: Speler::X,
    }];
    let suggestie = suggereer_zetten(zetten);
    let verwachte_suggesties = vec![
        Zet {
            x: 0,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 2,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 2,
            speler: Speler::O,
        },
        Zet {
            x: 2,
            y: 2,
            speler: Speler::O,
        },
    ];
    assert_eq!(suggestie, verwachte_suggesties);
}

#[test]
fn zijde_zet_wordt_gesuggereerd() {
    let zetten = vec![
        Zet {
            x: 1,
            y: 1,
            speler: Speler::X,
        },
        Zet {
            x: 2,
            y: 2,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 0,
            speler: Speler::X,
        },
        Zet {
            x: 2,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 0,
            y: 2,
            speler: Speler::X,
        },
        Zet {
            x: 0,
            y: 1,
            speler: Speler::O,
        },
        Zet {
            x: 2,
            y: 1,
            speler: Speler::X,
        },
    ];
    let suggestie = suggereer_zetten(zetten);
    let verwachte_suggesties = vec![
        Zet {
            x: 1,
            y: 0,
            speler: Speler::O,
        },
        Zet {
            x: 1,
            y: 2,
            speler: Speler::O,
        },
    ];
    assert_eq!(suggestie, verwachte_suggesties);
}
