use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_johnny_mnemonic() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/jm_12r.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 34_000_030,
            label: Some("Player 1".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 2".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 3".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 4".into()),
        },
    ];
    assert_eq!(Some(expected), last_game);

    let champions = nvram.read_mode_champions()?;
    let expected = vec![
        ModeChampion {
            label: Some("Cyberpunk".to_string()),
            short_label: Some("Cyberpunk".to_string()),
            initials: Some("TWU".to_string()),
            score: None,
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #1".to_string()),
            short_label: Some("Powerdown #1".to_string()),
            initials: Some("ROG".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #2".to_string()),
            short_label: Some("Powerdown #2".to_string()),
            initials: Some("LED".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #3".to_string()),
            short_label: Some("Powerdown #3".to_string()),
            initials: Some("LFS".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #4".to_string()),
            short_label: Some("Powerdown #4".to_string()),
            initials: Some("JAP".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        // below are not shown as the initials are "   "
        ModeChampion {
            label: Some("Masters of Powerdown #5".to_string()),
            short_label: Some("Powerdown #5".to_string()),
            initials: Some("   ".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #6".to_string()),
            short_label: Some("Powerdown #6".to_string()),
            initials: Some("   ".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #7".to_string()),
            short_label: Some("Powerdown #7".to_string()),
            initials: Some("   ".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #8".to_string()),
            short_label: Some("Powerdown #8".to_string()),
            initials: Some("   ".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #9".to_string()),
            short_label: Some("Powerdown #9".to_string()),
            initials: Some("   ".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("Masters of Powerdown #10".to_string()),
            short_label: Some("Powerdown #10".to_string()),
            initials: Some("   ".to_string()),
            score: Some(100_000_000),
            suffix: None,
            timestamp: None,
        },
    ];
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = vec![
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "GAG".to_string(),
            score: 9_000_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "TMK".to_string(),
            score: 8_000_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "ZAB".to_string(),
            score: 7_500_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "LEU".to_string(),
            score: 6_000_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "JON".to_string(),
            score: 5_500_000_000,
        },
    ];

    assert_eq!(expected, scores);
    Ok(())
}
