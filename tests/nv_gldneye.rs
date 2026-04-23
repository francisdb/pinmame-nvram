use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_goldeneye() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/gldneye.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected_last_game = vec![
        LastGamePlayer {
            score: 104019730,
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
        LastGamePlayer {
            score: 0,
            label: Some("Player 5".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 6".to_string()),
        },
    ];
    assert_eq!(Some(expected_last_game), last_game);

    let scores = nvram.read_highscores()?;
    let expected = vec![
        HighScore {
            label: Some("First".to_string()),
            short_label: Some("#1".to_string()),
            initials: "WMT".to_string(),
            score: 2_400_000_000,
        },
        HighScore {
            label: Some("Second".to_string()),
            short_label: Some("#2".to_string()),
            initials: "JRP".to_string(),
            score: 2_100_000_000,
        },
        HighScore {
            label: Some("Third".to_string()),
            short_label: Some("#3".to_string()),
            initials: "RFH".to_string(),
            score: 1_950_000_000,
        },
        HighScore {
            label: Some("Fourth".to_string()),
            short_label: Some("#4".to_string()),
            initials: "BTB".to_string(),
            score: 1_800_000_000,
        },
        HighScore {
            label: Some("Fifth".to_string()),
            short_label: Some("#5".to_string()),
            initials: "NF ".to_string(),
            score: 1_650_000_000,
        },
        HighScore {
            label: Some("Sixth".to_string()),
            short_label: Some("#6".to_string()),
            initials: "DAY".to_string(),
            score: 1_500_000_000,
        },
    ];

    assert_eq!(expected, scores);
    Ok(())
}
