use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_theatre_of_magic() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/tom_13.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 91_050_050,
            label: Some("Player 1".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 2".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 3".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 4".into()),
        },
    ]);
    assert_eq!(Some(expected), last_game);

    let champions = nvram.read_mode_champions()?;
    let expected = Vec::from([
        ModeChampion {
            label: Some("BUY-IN SCORE #1".into()),
            short_label: Some("BIS-1".into()),
            initials: "WWW".into(),
            score: Some(1_000_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("BUY-IN SCORE #2".into()),
            short_label: Some("BIS-2".into()),
            initials: "BMC".into(),
            score: Some(980_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("BUY-IN SCORE #3".into()),
            short_label: Some("BIS-3".into()),
            initials: "CMJ".into(),
            score: Some(960_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("BUY-IN SCORE #4".into()),
            short_label: Some("BIS-4".into()),
            initials: "ASR".into(),
            score: Some(940_000_000),
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
            initials: "JBJ".to_string(),
            score: 800_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "POP".to_string(),
            score: 640_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "ZAB".to_string(),
            score: 630_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "JWS".to_string(),
            score: 620_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "LTD".to_string(),
            score: 610_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
