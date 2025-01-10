use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_high_speed() -> io::Result<()> {
    let mut nvram = Nvram::open_local(Path::new("testdata/hs_l4.nv"))?.unwrap();

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
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "SSR".to_string(),
            score: 4_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "LED".to_string(),
            score: 3_500_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "ML ".to_string(),
            score: 3_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "JWA".to_string(),
            score: 2_500_000,
        },
    ]);
    assert_eq!(expected, scores);

    let game_state = nvram.read_game_state()?;
    // >        "current_ball": "0",
    // >        "hideout_jackpot": "280000",
    // >        "extra_balls": "0",
    // >        "tilt_warnings": "0",
    // >        "bonus_hold": "0",
    // >        "bonus": "Bits encoding not implemented",
    // >        "bonusX": "Bits encoding not implemented",
    // >        "current_player": "0",
    // >        "player_count": "0",
    // >        "match": "0",
    let expected = HashMap::from([
        ("current_ball".into(), "0".into()),
        ("hideout_jackpot".into(), "280000".into()),
        ("extra_balls".into(), "0".into()),
        ("tilt_warnings".into(), "0".into()),
        ("bonus_hold".into(), "0".into()),
        ("bonus".into(), "Bits encoding not implemented".into()),
        ("bonusX".into(), "Bits encoding not implemented".into()),
        ("current_player".into(), "0".into()),
        ("player_count".into(), "0".into()),
        ("match".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    Ok(())
}
