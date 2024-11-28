use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
#[ignore = "Not sure where this goes wrong"]
fn test_batman_last_game() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/btmn_106.nv"))?.unwrap();

    // FIXME this is pobably wrong my last game was 41_180?
    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 41_180,
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

    Ok(())
}

#[test]
fn test_batman() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/btmn_106.nv"))?.unwrap();

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Dark Knight".to_string()),
            short_label: Some("GC".to_string()),
            initials: "TIM".to_string(),
            score: 30_000_000,
        },
        HighScore {
            label: Some("1st Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "DAN".to_string(),
            score: 25_000_000,
        },
        HighScore {
            label: Some("2nd Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "JEK".to_string(),
            score: 20_000_000,
        },
        HighScore {
            label: Some("3rd Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: " NF".to_string(),
            score: 18_000_000,
        },
        HighScore {
            label: Some("4th Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BLS".to_string(),
            score: 16_000_000,
        },
        HighScore {
            label: Some("5th Place".to_string()),
            short_label: Some("5th".to_string()),
            initials: "HEC".to_string(),
            score: 14_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
