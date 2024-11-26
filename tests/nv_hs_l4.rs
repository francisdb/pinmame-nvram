use pinmame_nvram::{HighScore, Nvram};
use std::io;
use std::path::Path;

#[test]
fn test_high_speed() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/hs_l4.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "SSR".to_string(),
            score: 4_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "LED".to_string(),
            score: 3_500_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "ML ".to_string(),
            score: 3_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "JWA".to_string(),
            score: 2_500_000,
        },
    ]);

    Ok(pretty_assertions::assert_eq!(expected, scores))
}
