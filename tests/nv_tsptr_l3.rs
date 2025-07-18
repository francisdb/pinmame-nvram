use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_transporter_the_rescue() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/tsptr_l3.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 0,
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
            initials: "COL".to_string(),
            score: 7_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "DAN".to_string(),
            score: 6_500_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "TIM".to_string(),
            score: 6_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "R/J".to_string(),
            score: 5_500_000,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}
