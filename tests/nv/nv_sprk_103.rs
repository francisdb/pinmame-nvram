use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_south_park() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/sprk_103.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected_last_game = vec![
        LastGamePlayer {
            score: 1135780,
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
            label: Some("#1".to_string()),
            short_label: None,
            initials: "JEK       ".to_string(),
            score: 400_000_000,
        },
        HighScore {
            label: Some("#2".to_string()),
            short_label: None,
            initials: "KIM       ".to_string(),
            score: 375_000_000,
        },
        HighScore {
            label: Some("#3".to_string()),
            short_label: None,
            initials: "CMK       ".to_string(),
            score: 350_000_000,
        },
        HighScore {
            label: Some("#4".to_string()),
            short_label: None,
            initials: "DLK       ".to_string(),
            score: 325_000_000,
        },
        HighScore {
            label: Some("#5".to_string()),
            short_label: None,
            initials: "KLK       ".to_string(),
            score: 300_000_000,
        },
        HighScore {
            label: Some("#6".to_string()),
            short_label: None,
            initials: "JOE       ".to_string(),
            score: 275_000_000,
        },
        HighScore {
            label: Some("#7".to_string()),
            short_label: None,
            initials: "NF        ".to_string(),
            score: 250_000_000,
        },
        HighScore {
            label: Some("#8".to_string()),
            short_label: None,
            initials: "DAY       ".to_string(),
            score: 225_000_000,
        },
        HighScore {
            label: Some("#9".to_string()),
            short_label: None,
            initials: "KRT       ".to_string(),
            score: 200_000_000,
        },
        HighScore {
            label: Some("#10".to_string()),
            short_label: None,
            initials: "RFH       ".to_string(),
            score: 175_000_000,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}
