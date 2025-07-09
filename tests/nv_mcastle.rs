use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_magic_castle() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/mcastle.nv"))?.unwrap();

    // 8 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("credits".into(), "93".into()),
        ("scores.0".into(), "131430".into()),
        ("scores.1".into(), "113210".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 131_430,
            label: Some("Player 1".to_string()),
        },
        LastGamePlayer {
            score: 113_210,
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
    let expected = Vec::from([HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 7_500_000,
    }]);
    assert_eq!(expected, scores);

    Ok(())
}
