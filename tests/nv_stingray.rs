use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_stingray() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/stingray.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 301_300,
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

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 820_710,
    }]);
    assert_eq!(expected, scores);

    Ok(())
}
