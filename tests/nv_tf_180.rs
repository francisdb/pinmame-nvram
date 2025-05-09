use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_transformers() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/tf_180.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 584_950,
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
            label: Some("Combo".into()),
            short_label: Some("Combo".into()),
            initials: Some("LON".into()),
            score: Some(20),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Best Combo Champion".into()),
            short_label: Some("Best Combo".into()),
            initials: Some("LON".into()),
            score: Some(327681),
            suffix: Some("-WAY".into()),
            timestamp: None,
        },
    ]);
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Autobot Grand Champion".to_string()),
            short_label: Some("AGC".to_string()),
            initials: "OPT".to_string(),
            score: 75_000_000,
        },
        HighScore {
            label: Some("Autobot #1".to_string()),
            short_label: Some("A#1".to_string()),
            initials: "JAZ".to_string(),
            score: 55_000_000,
        },
        HighScore {
            label: Some("Autobot #2".to_string()),
            short_label: Some("A#2".to_string()),
            initials: "PWL".to_string(),
            score: 40_000_000,
        },
        HighScore {
            label: Some("Autobot #3".to_string()),
            short_label: Some("A#3".to_string()),
            initials: "IRN".to_string(),
            score: 30_000_000,
        },
        HighScore {
            label: Some("Autobot #4".to_string()),
            short_label: Some("A#4".to_string()),
            initials: "BEE".to_string(),
            score: 25_000_000,
        },
        HighScore {
            label: Some("Decepticon Grand Champion".to_string()),
            short_label: Some("DGC".to_string()),
            initials: "MEG".to_string(),
            score: 75_000_000,
        },
        HighScore {
            label: Some("Decepticon #1".to_string()),
            short_label: Some("D#1".to_string()),
            initials: "STR".to_string(),
            score: 55_000_000,
        },
        HighScore {
            label: Some("Decepticon #2".to_string()),
            short_label: Some("D#2".to_string()),
            initials: "SND".to_string(),
            score: 40_000_000,
        },
        HighScore {
            label: Some("Decepticon #3".to_string()),
            short_label: Some("D#3".to_string()),
            initials: "SHK".to_string(),
            score: 30_000_000,
        },
        HighScore {
            label: Some("Decepticon #4".to_string()),
            short_label: Some("D#4".to_string()),
            initials: "BLK".to_string(),
            score: 25_000_000,
        },
    ]);

    assert_eq!(expected, scores);
    Ok(())
}
