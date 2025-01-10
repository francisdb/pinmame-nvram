use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_harley_davidson() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/hd_l3.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 0,
            label: Some("Player 1".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 2".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 3".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 4".to_string()),
        },
    ]);
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "BSO".to_string(),
            score: 15_000_000,
        },
        HighScore {
            label: Some("1st".to_string()),
            short_label: Some("1".to_string()),
            initials: "JRS".to_string(),
            score: 12_000_000,
        },
        HighScore {
            label: Some("2nd".to_string()),
            short_label: Some("2".to_string()),
            initials: "DWF".to_string(),
            score: 11_000_000,
        },
        HighScore {
            label: Some("3rd".to_string()),
            short_label: Some("3".to_string()),
            initials: "MWS".to_string(),
            score: 10_000_000,
        },
        HighScore {
            label: Some("4th".to_string()),
            short_label: Some("4".to_string()),
            initials: "GLH".to_string(),
            score: 9_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
