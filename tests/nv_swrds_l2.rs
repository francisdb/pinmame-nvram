use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_swords_of_fury() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/swrds_l2.nv"))?.unwrap();

    // replay at: 2_000_000
    // Jackpot bonus: 505_000

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("scores.0".into(), "25030".into()),
        ("scores.1".into(), "0".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
        ("credits".into(), "12".into()),
        ("current_ball".into(), "1".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 25_030,
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
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "AMK".to_string(),
            score: 5_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "DKL".to_string(),
            score: 4_500_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "DTW".to_string(),
            score: 4_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BLS".to_string(),
            score: 3_500_000,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}
