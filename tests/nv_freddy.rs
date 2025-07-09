use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_freddy_a_nightmare_on_elm_street() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/freddy.nv"))?.unwrap();

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
            label: Some("Dream Master".to_string()),
            short_label: Some("#1".to_string()),
            initials: "BRI".to_string(),
            score: 160_000_000,
        },
        HighScore {
            label: Some("Dream Warriors".to_string()),
            short_label: Some("#2".to_string()),
            initials: "LVS".to_string(),
            score: 140_000_000,
        },
        HighScore {
            label: Some("Dream Warriors".to_string()),
            short_label: Some("#3".to_string()),
            initials: "CIN".to_string(),
            score: 120_000_000,
        },
        HighScore {
            label: Some("Dream Warriors".to_string()),
            short_label: Some("#4".to_string()),
            initials: "JON".to_string(),
            score: 100_000_000,
        },
        HighScore {
            label: Some("Dream Warriors".to_string()),
            short_label: Some("#5".to_string()),
            initials: "GIL".to_string(),
            score: 80_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
