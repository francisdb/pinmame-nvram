use pinmame_nvram::{HighScore, Nvram};
use std::io;
use std::path::Path;

#[test]
fn test_johnny_mnemonic() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/jm_12r.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
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
    ]);

    Ok(pretty_assertions::assert_eq!(expected, scores))
}
