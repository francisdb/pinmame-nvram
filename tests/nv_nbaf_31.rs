use pinmame_nvram::{HighScore, Nvram};
use std::io;
use std::path::Path;

#[test]
fn test_nba_fastbreak() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/nbaf_31.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "GG ".to_string(),
            score: 150,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "TMK".to_string(),
            score: 130,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "KCQ".to_string(),
            score: 110,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "CAT".to_string(),
            score: 90,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "MAS".to_string(),
            score: 70,
        },
    ]);

    Ok(pretty_assertions::assert_eq!(expected, scores))
}
