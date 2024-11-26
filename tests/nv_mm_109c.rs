use pinmame_nvram::{HighScore, Nvram};
use std::io;
use std::path::Path;

#[test]
fn test_medieval_madness() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/mm_109c.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "SLL".to_string(),
            score: 52_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "BRE".to_string(),
            score: 44_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "LFS".to_string(),
            score: 40_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "ZAP".to_string(),
            score: 36_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "RCF".to_string(),
            score: 32_000_000,
        },
    ]);

    Ok(pretty_assertions::assert_eq!(expected, scores))
}
