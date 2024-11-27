use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_no_fear_dangerous_sports() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/nf_23x.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "SSR".to_string(),
            score: 2_000_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "MAT".to_string(),
            score: 1_600_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "ZAP".to_string(),
            score: 1_400_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "DAN".to_string(),
            score: 1_200_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "PJP".to_string(),
            score: 1_000_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
