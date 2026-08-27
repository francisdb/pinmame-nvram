use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_earthshaker_la3() -> io::Result<()> {
    // let roms = ["esha_la3", "esha_l4c", "esha_ma3"];
    //
    let mut nvram = Nvram::open(Path::new("testdata/esha_la3.nv"))?.unwrap();

    // Replay at 3_000_000
    // High score players named "Movers and Shakers"

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("scores.0".into(), "869880".into()),
        ("scores.1".into(), "0".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
        ("credits".into(), "5".into()),
        ("current_ball".into(), "3".into()),
        ("current_player".into(), "0".into()),
        ("tilt_warnings".into(), "0".into()),
        ("tilted".into(), "false".into()),
        ("extra_balls".into(), "0".into()),
        ("ball_count".into(), "3".into()),
        ("max_credits".into(), "10".into()),
        ("game_over".into(), "false".into()),
        ("player_count".into(), "0".into()),
        ("free_play".into(), "false".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 869_880,
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
    ];
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = default_highscores();
    assert_eq!(expected, scores);

    Ok(())
}

#[test]
fn test_earthshaker_l4c() -> io::Result<()> {
    // let roms = ["esha_la3", "esha_l4c", "esha_ma3"];
    //
    let mut nvram = Nvram::open(Path::new("testdata/esha_l4c.nv"))?.unwrap();

    // Replay at 3_000_000
    // High score players named "Movers and Shakers"

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("scores.0".into(), "163020".into()),
        ("scores.1".into(), "0".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
        ("credits".into(), "3".into()),
        ("current_ball".into(), "1".into()),
        ("extra_balls".into(), "0".into()),
        ("tilt_warnings".into(), "0".into()),
        ("tilted".into(), "false".into()),
        ("ball_count".into(), "3".into()),
        ("player_count".into(), "0".into()),
        ("current_player".into(), "0".into()),
        ("max_credits".into(), "10".into()),
        ("game_over".into(), "false".into()),
        ("free_play".into(), "false".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 163_020,
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
    ];
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = default_highscores();
    assert_eq!(expected, scores);

    Ok(())
}

#[test]
fn test_earthshaker_ma3() -> io::Result<()> {
    // let roms = ["esha_la3", "esha_l4c", "esha_ma3"];
    //
    let mut nvram = Nvram::open(Path::new("testdata/esha_ma3.nv"))?.unwrap();

    // Replay at 3_000_000
    // High score players named "Movers and Shakers"

    let game_state = nvram.read_game_state()?;
    //
    let expected = HashMap::from([
        ("scores.0".into(), "67020".into()),
        ("scores.1".into(), "0".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
        ("credits".into(), "7".into()),
        ("current_ball".into(), "1".into()),
        ("extra_balls".into(), "0".into()),
        ("tilt_warnings".into(), "0".into()),
        ("tilted".into(), "false".into()),
        ("ball_count".into(), "3".into()),
        ("player_count".into(), "1".into()),
        ("current_player".into(), "0".into()),
        ("max_credits".into(), "10".into()),
        ("game_over".into(), "false".into()),
        ("free_play".into(), "false".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 67_020,
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
    ];
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = default_highscores();
    assert_eq!(expected, scores);

    Ok(())
}

fn default_highscores() -> Vec<HighScore> {
    vec![
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "JON".to_string(),
            score: 6_500_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "LED".to_string(),
            score: 6_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "PJ ".to_string(),
            score: 5_500_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "SJO".to_string(),
            score: 5_000_000,
        },
    ]
}
