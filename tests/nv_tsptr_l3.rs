use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_transporter_the_rescue() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/tsptr_l3.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "COL".to_string(),
            score: 7_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "DAN".to_string(),
            score: 6_500_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "TIM".to_string(),
            score: 6_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "R/J".to_string(),
            score: 5_500_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
