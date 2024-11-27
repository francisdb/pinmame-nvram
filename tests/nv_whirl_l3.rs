use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_whirlwind() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/whirl_l3.nv"))?.unwrap();

    let champions = nvram.read_mode_champions()?;
    assert_eq!(None, champions);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Champion".to_string()),
            short_label: Some("Champ".to_string()),
            initials: "HHR".to_string(),
            score: 10_000_000,
        },
        HighScore {
            label: Some("1st".to_string()),
            short_label: Some("#1".to_string()),
            initials: "JCY".to_string(),
            score: 6_000_000,
        },
        HighScore {
            label: Some("2nd".to_string()),
            short_label: Some("#2".to_string()),
            initials: "JRK".to_string(),
            score: 5_500_000,
        },
        HighScore {
            label: Some("3rd".to_string()),
            short_label: Some("#3".to_string()),
            initials: "CPG".to_string(),
            score: 5_000_000,
        },
        HighScore {
            label: Some("4th".to_string()),
            short_label: Some("#4".to_string()),
            initials: "PFZ".to_string(),
            score: 4_500_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
