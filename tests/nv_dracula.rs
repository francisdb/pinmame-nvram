use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_dracula_last_game() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dracula.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 32520,
            label: None,
        },
        LastGamePlayer {
            score: 22510,
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

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("scores.0".into(), "32520".into()),
        ("scores.1".into(), "22510".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
        ("current_ball".into(), "0".into()),
        ("credits".into(), "0".into()),
        ("match".into(), "70".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    Ok(())
}

#[test]
fn test_dracula() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dracula.nv"))?.unwrap();

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 440_040,
    }]);
    assert_eq!(expected, scores);

    Ok(())
}
