use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_time_machine() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/tmachzac.nv"))?.unwrap();

    // 7 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("credits".into(), "47".into()),
        ("scores.0".into(), "580340".into()),
        ("scores.1".into(), "43100".into()),
        ("scores.2".into(), "66500".into()),
        ("scores.3".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 580_340,
            label: Some("Player 1".into()),
        },
        LastGamePlayer {
            score: 43_100,
            label: Some("Player 2".into()),
        },
        LastGamePlayer {
            score: 66_500,
            label: Some("Player 3".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 4".into()),
        },
    ];
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = vec![HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 4_000_000,
    }];
    assert_eq!(expected, scores);

    Ok(())
}
