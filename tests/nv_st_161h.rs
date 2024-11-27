use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
#[ignore = "Only st_162h.nv is available"]
fn test_star_trek_stern() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/st_161h.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "PML".to_string(),
            score: 50_000_000,
        },
        HighScore {
            label: Some("1st".to_string()),
            short_label: Some("#1".to_string()),
            initials: "LNK".to_string(),
            score: 31_000_000,
        },
        HighScore {
            label: Some("2nd".to_string()),
            short_label: Some("#2".to_string()),
            initials: "JRK".to_string(),
            score: 25_000_000,
        },
        HighScore {
            label: Some("3rd".to_string()),
            short_label: Some("#3".to_string()),
            initials: "J Y".to_string(),
            score: 22_000_000,
        },
        HighScore {
            label: Some("4th".to_string()),
            short_label: Some("#4".to_string()),
            initials: "C G".to_string(),
            score: 18_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
