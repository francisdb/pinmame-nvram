use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_batman() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/btmn_106.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Dark Knight".to_string()),
            short_label: Some("GC".to_string()),
            initials: "TIM".to_string(),
            score: 30_000_000,
        },
        HighScore {
            label: Some("1st Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "DAN".to_string(),
            score: 25_000_000,
        },
        HighScore {
            label: Some("2nd Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "JEK".to_string(),
            score: 20_000_000,
        },
        HighScore {
            label: Some("3rd Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: " NF".to_string(),
            score: 18_000_000,
        },
        HighScore {
            label: Some("4th Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BLS".to_string(),
            score: 16_000_000,
        },
        HighScore {
            label: Some("5th Place".to_string()),
            short_label: Some("5th".to_string()),
            initials: "HEC".to_string(),
            score: 14_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
