use pinmame_nvram::{HighScore, Nvram};
use std::io;
use std::path::Path;

#[test]
fn test_goldeneye() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/gldneye.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "WMT".to_string(),
            score: 2_400_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "JRP".to_string(),
            score: 2_100_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "RFH".to_string(),
            score: 1_950_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BTB".to_string(),
            score: 1_800_000_000,
        },
        HighScore {
            label: Some("Fifth Place".to_string()),
            short_label: Some("5th".to_string()),
            initials: "NF ".to_string(),
            score: 1_650_000_000,
        },
        HighScore {
            label: Some("Sixth Place".to_string()),
            short_label: Some("6th".to_string()),
            initials: "DAY".to_string(),
            score: 1_500_000_000,
        },
    ]);

    Ok(pretty_assertions::assert_eq!(expected, scores))
}
