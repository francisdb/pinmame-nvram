use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_bad_cats() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/bcats_l5.nv"))?.unwrap();

    // replay at 3_000_000
    // jackpot worth 20_080_000

    let game_state = nvram.read_game_state()?;

    let expected = HashMap::from([
        ("scores.0".into(), "1520990".into()),
        ("scores.1".into(), "0".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
        ("credits".into(), "6".into()),
        ("current_ball".into(), "0".into()),
        ("current_player".into(), "0".into()),
        ("tilt_warnings".into(), "0".into()),
        ("tilted".into(), "false".into()),
        ("extra_balls".into(), "0".into()),
        ("ball_count".into(), "3".into()),
        ("max_credits".into(), "10".into()),
        ("game_over".into(), "true".into()),
        ("player_count".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 1520990,
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
    let expected = vec![
        HighScore {
            label: Some("Baddest Cat 1".to_string()),
            short_label: Some("1st".to_string()),
            initials: "SOM".to_string(),
            score: 8_697_720,
        },
        HighScore {
            label: Some("Baddest Cat 2".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "BSO".to_string(),
            score: 6_000_000,
        },
        HighScore {
            label: Some("Baddest Cat 3".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "PVA".to_string(),
            score: 5_500_000,
        },
        HighScore {
            label: Some("Baddest Cat 4".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BAD".to_string(),
            score: 5_000_000,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}
