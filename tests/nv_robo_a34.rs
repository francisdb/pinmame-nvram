use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_robocop() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/robo_a34.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 130_240,
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

    let champions = nvram.read_mode_champions()?;
    let expected = vec![ModeChampion {
        label: Some("Jump Master".to_string()),
        short_label: Some("Jump Master".to_string()),
        initials: Some("MJW".to_string()),
        score: Some(5),
        suffix: Some(" Jumps".to_string()),
        timestamp: None,
    }];
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = vec![
        HighScore {
            label: Some("Commander".to_string()),
            short_label: Some("GC".to_string()),
            initials: "BMW".to_string(),
            score: 4_000_000,
        },
        HighScore {
            label: Some("1st Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "TEB".to_string(),
            score: 3_500_000,
        },
        HighScore {
            label: Some("2nd Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "ERB".to_string(),
            score: 3_000_000,
        },
        HighScore {
            label: Some("3rd Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "DAS".to_string(),
            score: 2_500_000,
        },
    ];

    assert_eq!(expected, scores);
    Ok(())
}
