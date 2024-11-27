use pinmame_nvram::{HighScore, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_attack_from_mars() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/afm_113b.nv"))?.unwrap();

    let champions = nvram.read_mode_champions()?;
    let expected = Vec::from([
        ModeChampion {
            label: Some("Martian Champion".to_string()),
            short_label: Some("Martian Champ".to_string()),
            initials: "LFS".to_string(),
            score: Some(20),
            suffix: Some(" Martians Destroyed".to_string()),
            timestamp: None,
        },
        ModeChampion {
            label: Some("Ruler of the Universe".to_string()),
            short_label: Some("Rule the Universe".to_string()),
            initials: "TEX".to_string(),
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
    ]);

    Ok(assert_eq!(expected, scores))
}
