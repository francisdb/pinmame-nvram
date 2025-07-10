use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_shooting_the_rapids() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/strapids.nv"))?.unwrap();

    // 6 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("credits".into(), "17".into()),
        ("scores.0".into(), "114490".into()),
        ("scores.1".into(), "65920".into()),
        ("scores.2".into(), "53440".into()),
        ("scores.3".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 114_490,
            label: Some("Player 1".into()),
        },
        LastGamePlayer {
            score: 65_920,
            label: Some("Player 2".into()),
        },
        LastGamePlayer {
            score: 53_440,
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
        score: 120_140,
    }];
    assert_eq!(expected, scores);

    Ok(())
}
