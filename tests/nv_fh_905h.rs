use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_funhouse_905h() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/fh_905h.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 756_330,
            label: Some("Player 1".to_string()),
        },
        LastGamePlayer {
            score: 605_990,
            label: Some("Player 2".to_string()),
        },
        LastGamePlayer {
            score: 1_014_990,
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
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "LED".to_string(),
            score: 15_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "PML".to_string(),
            score: 12_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "EAE".to_string(),
            score: 11_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "J K".to_string(),
            score: 10_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "HEY".to_string(),
            score: 9_000_000,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}

#[test]
fn test_funhouse_906h() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/fh_906h.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 2_170_790,
            label: Some("Player 1".to_string()),
        },
        LastGamePlayer {
            score: 1_880_850,
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
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "LED".to_string(),
            score: 15_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "PML".to_string(),
            score: 12_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "EAE".to_string(),
            score: 11_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "J K".to_string(),
            score: 10_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "HEY".to_string(),
            score: 9_000_000,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}
