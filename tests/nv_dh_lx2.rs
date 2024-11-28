use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_dirty_harry() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dh_lx2.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 12_000_000,
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

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "CJS".to_string(),
            score: 900_000_000,
        },
        HighScore {
            label: Some("#1".to_string()),
            short_label: Some("#1".to_string()),
            initials: "BSO".to_string(),
            score: 800_000_000,
        },
        HighScore {
            label: Some("#2".to_string()),
            short_label: Some("#2".to_string()),
            initials: "BIL".to_string(),
            score: 700_000_000,
        },
        HighScore {
            label: Some("#3".to_string()),
            short_label: Some("#3".to_string()),
            initials: "VJP".to_string(),
            score: 600_000_000,
        },
        HighScore {
            label: Some("#4".to_string()),
            short_label: Some("#4".to_string()),
            initials: "PAT".to_string(),
            score: 500_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}