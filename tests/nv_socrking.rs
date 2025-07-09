use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_soccer_kings() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/socrking.nv"))?.unwrap();

    // 8 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("credits".into(), "71".into()),
        ("scores.0".into(), "137680".into()),
        ("scores.1".into(), "414950".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 137_680,
            label: Some("Player 1".into()),
        },
        LastGamePlayer {
            score: 414_950,
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
        score: 13_500_000,
    }]);
    assert_eq!(expected, scores);

    Ok(())
}
