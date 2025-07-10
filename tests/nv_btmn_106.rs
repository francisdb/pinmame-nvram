use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_batman() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/btmn_106.nv"))?.unwrap();

    // let game_state = nvram.read_game_state()?;
    // let expected = HashMap::from([("credits".into(), "11".into())]);
    // assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 339_930,
            label: Some("Player 1".to_string()),
        },
        LastGamePlayer {
            score: 1_868_230,
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
            label: Some("Dark Knight".to_string()),
            short_label: Some("GC".to_string()),
            initials: "TIM".to_string(),
            score: 30_000_000,
        },
        HighScore {
            label: Some("1st Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "DAN".to_string(),
            score: 25_000_000,
        },
        HighScore {
            label: Some("2nd Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "JEK".to_string(),
            score: 20_000_000,
        },
        HighScore {
            label: Some("3rd Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: " NF".to_string(),
            score: 18_000_000,
        },
        HighScore {
            label: Some("4th Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BLS".to_string(),
            score: 16_000_000,
        },
        HighScore {
            label: Some("5th Place".to_string()),
            short_label: Some("5th".to_string()),
            initials: "HEC".to_string(),
            score: 14_000_000,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}
