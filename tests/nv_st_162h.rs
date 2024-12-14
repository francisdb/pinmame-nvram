use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_star_trek_stern() -> io::Result<()> {
    // TODO add test for st_161h.nv
    // TODO fix last highscore value when this is implemented: https://github.com/tomlogic/pinmame-nvram-maps/issues/34
    let mut nvram = Nvram::open(Path::new("testdata/st_162h.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
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
            initials: "SSR".to_string(),
            score: 75_000_000,
        },
        HighScore {
            label: Some("High Score #1".to_string()),
            short_label: Some("#1".to_string()),
            initials: "LON".to_string(),
            score: 55_000_000,
        },
        HighScore {
            label: Some("High Score #2".to_string()),
            short_label: Some("#2".to_string()),
            initials: "GGF".to_string(),
            score: 40_000_000,
        },
        HighScore {
            label: Some("High Score #3".to_string()),
            short_label: Some("#3".to_string()),
            initials: "JMR".to_string(),
            score: 30_000_000,
        },
        HighScore {
            label: Some("High Score #4".to_string()),
            short_label: Some("#4".to_string()),
            initials: "T".to_string(),
            score: 25_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
