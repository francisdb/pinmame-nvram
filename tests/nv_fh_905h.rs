use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
#[ignore = "only fh_l9.nv is set up"]
fn test_funhouse() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/fh_905h.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "EJB".to_string(),
            score: 4_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "CLF".to_string(),
            score: 3_800_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "JCD".to_string(),
            score: 3_600_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "KWD".to_string(),
            score: 3_400_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
