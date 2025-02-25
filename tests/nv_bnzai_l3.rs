use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_banzai_run() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/bnzai_l3.nv"))?.unwrap();

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("scores.0".into(), "134190".into()),
        ("scores.1".into(), "0".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "40002".into()),
        ("credits".into(), "4".into()),
        ("current_ball".into(), "2".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 134190,
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
            // TODO is this correct? Only first and last player have scores.
            score: 40002,
            label: Some("Player 4".to_string()),
        },
    ]);
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "KJF".to_string(),
            score: 4_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "J C".to_string(),
            score: 3_800_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "JEJ".to_string(),
            score: 3_600_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "SJO".to_string(),
            score: 3_400_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
