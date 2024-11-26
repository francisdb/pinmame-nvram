use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_elvira_and_the_party_monsters() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/eatpm_l4.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "LOU".to_string(),
            score: 8_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "CLS".to_string(),
            score: 7_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "LJR".to_string(),
            score: 6_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "ROG".to_string(),
            score: 5_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
