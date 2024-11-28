use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_doctor_who() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dw_l2.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 4_262_570,
            label: None,
        },
        LastGamePlayer {
            score: 0,
            label: None,
        },
        LastGamePlayer {
            score: 0,
            label: None,
        },
        LastGamePlayer {
            score: 0,
            label: None,
        },
    ]);
    assert_eq!(Some(expected), last_game);

    let champions = nvram.read_mode_champions()?;
    let expected = Vec::from([
        ModeChampion {
            label: Some("Loop Champion".to_string()),
            short_label: Some("Loop Champ".to_string()),
            initials: "WHO".to_string(),
            score: Some(6),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Highest Davros Wave".to_string()),
            short_label: Some("Davros Champ".to_string()),
            initials: "WHO".to_string(),
            score: Some(0),
            suffix: None,
            timestamp: None,
        },
    ]);
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "WHO".to_string(),
            score: 300_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "BSO".to_string(),
            score: 250_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "BIL".to_string(),
            score: 200_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "HEY".to_string(),
            score: 150_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "S  ".to_string(),
            score: 100_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
