use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_tales_of_the_arabian_nights() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/totan_14.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 678040,
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
            initials: "KOZ".to_string(),
            score: 9_250_000,
        },
        HighScore {
            label: Some("1st".to_string()),
            short_label: Some("#1".to_string()),
            initials: "POP".to_string(),
            score: 9_000_000,
        },
        HighScore {
            label: Some("2nd".to_string()),
            short_label: Some("#2".to_string()),
            initials: "MAX".to_string(),
            score: 8_750_000,
        },
        HighScore {
            label: Some("3rd".to_string()),
            short_label: Some("#3".to_string()),
            initials: "SKA".to_string(),
            score: 8_500_000,
        },
        HighScore {
            label: Some("4th".to_string()),
            short_label: Some("#4".to_string()),
            initials: "ZAB".to_string(),
            score: 8_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
