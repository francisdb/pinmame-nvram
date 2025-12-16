use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_dracula_last_game() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dracula.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 32520,
            label: Some("Final P1".to_string()),
        },
        LastGamePlayer {
            score: 22510,
            label: Some("Final P2".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Final P3".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Final P4".to_string()),
        },
    ];
    assert_eq!(Some(expected), last_game);

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("scores.0".into(), "32520".into()),
        ("scores.1".into(), "22510".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
        ("credits".into(), "0".into()),
        ("match".into(), "70".into()),
        ("game_over".into(), "false".into()),
        ("tilted".into(), "false".into()),
        ("final_scores.0".into(), "32520".into()),
        ("final_scores.1".into(), "22510".into()),
        ("final_scores.2".into(), "0".into()),
        ("final_scores.3".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    Ok(())
}

#[test]
fn test_dracula() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dracula.nv"))?.unwrap();

    let scores = nvram.read_highscores()?;
    let expected = vec![HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 440_040,
    }];
    assert_eq!(expected, scores);

    Ok(())
}
