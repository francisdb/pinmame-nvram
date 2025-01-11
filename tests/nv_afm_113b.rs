use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_attack_from_mars() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/afm_113b.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 51_477_300,
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
    let expected = Vec::from([
        ModeChampion {
            label: Some("Martian Champion".to_string()),
            short_label: Some("Martian Champ".to_string()),
            initials: Some("LFS".to_string()),
            score: Some(20),
            suffix: Some(" Martians Destroyed".to_string()),
            timestamp: None,
        },
        ModeChampion {
            label: Some("Ruler of the Universe".to_string()),
            short_label: Some("Rule the Universe".to_string()),
            initials: Some("TEX".to_string()),
            score: None,
            suffix: None,
            timestamp: Some("2023-11-07 00:14".to_string()),
        },
    ]);
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "SLL".to_string(),
            score: 7_500_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "BRE".to_string(),
            score: 7_000_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "LFS".to_string(),
            score: 6_500_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "RCF".to_string(),
            score: 6_000_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "DTW".to_string(),
            score: 5_500_000_000,
        },
        HighScore {
            label: Some("Buy-In Score #1".into()),
            short_label: Some("BI#1".into()),
            initials: "DWF".into(),
            score: 5000000000,
        },
        HighScore {
            label: Some("Buy-In Score #2".into()),
            short_label: Some("BI#2".into()),
            initials: "ASR".into(),
            score: 4500000000,
        },
        HighScore {
            label: Some("Buy-In Score #3".into()),
            short_label: Some("BI#3".into()),
            initials: "BCM".into(),
            score: 4000000000,
        },
        HighScore {
            label: Some("Buy-In Score #4".into()),
            short_label: Some("BI#4".into()),
            initials: "MOO".into(),
            score: 3500000000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
