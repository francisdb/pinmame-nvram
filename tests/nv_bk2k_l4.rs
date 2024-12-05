use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_black_knight_2000() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/bk2k_l4.nv"))?.unwrap();

    // Loop Champion JJJ 5 Loops
    // Jackpot Value 1_000_000
    // Replay at 3_500_000

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([("credits".into(), "8".into())]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 307_040,
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
            label: Some("Greatest Hero 1".to_string()),
            short_label: Some("1st".to_string()),
            initials: "SSR".to_string(),
            score: 5_500_000,
        },
        HighScore {
            label: Some("Greatest Hero 2".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "EJB".to_string(),
            score: 5_000_000,
        },
        HighScore {
            label: Some("Greatest Hero 3".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "DTW".to_string(),
            score: 4_500_000,
        },
        HighScore {
            label: Some("Greatest Hero 4".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BLS".to_string(),
            score: 4_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
