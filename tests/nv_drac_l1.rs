use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_bram_strokers_dracula() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/drac_l1.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 595_070,
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
    ]);
    assert_eq!(Some(expected), last_game);

    let champions = nvram.read_mode_champions()?;
    let expected = Vec::from([ModeChampion {
        label: Some("Loop Champion".to_string()),
        short_label: Some("LC".to_string()),
        initials: Some("CD ".to_string()),
        score: Some(6),
        suffix: None,
        timestamp: None,
    }]);
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Greatest Vampire Hunter".to_string()),
            short_label: Some("GVP".to_string()),
            initials: "CD".to_string(),
            score: 300_000_000,
        },
        HighScore {
            label: Some("Best Hunter #1".to_string()),
            short_label: Some("".to_string()),
            initials: "BSO".to_string(),
            score: 250_000_000,
        },
        HighScore {
            label: Some("Best Hunter #2".to_string()),
            short_label: None,
            initials: "BIL".to_string(),
            score: 200_000_000,
        },
        HighScore {
            label: Some("Best Hunter #3".to_string()),
            short_label: None,
            initials: "P H".to_string(),
            score: 150_000_000,
        },
        HighScore {
            label: Some("Best Hunter #4".to_string()),
            short_label: None,
            initials: "S  ".to_string(),
            score: 100_000_000,
        },
    ]);

    assert_eq!(expected, scores);
    Ok(())
}
