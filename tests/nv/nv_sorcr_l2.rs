use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_sorcerer() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/sorcr_l2.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 70330,
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
            label: Some("High Score #1".to_string()),
            short_label: None,
            initials: "".to_string(),
            score: 2_500_000,
        },
        HighScore {
            label: Some("High Score #2".to_string()),
            short_label: None,
            initials: "".to_string(),
            score: 2_000_000,
        },
        HighScore {
            label: Some("High Score #3".to_string()),
            short_label: None,
            initials: "".to_string(),
            score: 1_500_000,
        },
        HighScore {
            label: Some("High Score #4".to_string()),
            short_label: None,
            initials: "".to_string(),
            score: 1_100_000,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}
