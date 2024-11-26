use pinmame_nvram::{HighScore, Nvram};
use std::io;
use std::path::Path;

#[test]
fn test_south_park() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/sprk_103.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "JEK".to_string(),
            score: 400_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "KIM".to_string(),
            score: 375_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "CMK".to_string(),
            score: 350_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "DLK".to_string(),
            score: 325_000_000,
        },
        HighScore {
            label: Some("Fifth Place".to_string()),
            short_label: Some("5th".to_string()),
            initials: "KLK".to_string(),
            score: 300_000_000,
        },
        HighScore {
            label: Some("Sixth Place".to_string()),
            short_label: Some("6th".to_string()),
            initials: "JOE".to_string(),
            score: 275_000_000,
        },
        HighScore {
            label: Some("Seventh Place".to_string()),
            short_label: Some("7th".to_string()),
            initials: "NF ".to_string(),
            score: 250_000_000,
        },
        HighScore {
            label: Some("Eighth Place".to_string()),
            short_label: Some("8th".to_string()),
            initials: "DAY".to_string(),
            score: 225_000_000,
        },
        HighScore {
            label: Some("Ninth Place".to_string()),
            short_label: Some("9th".to_string()),
            initials: "KRT".to_string(),
            score: 200_000_000,
        },
        HighScore {
            label: Some("Tenth Place".to_string()),
            short_label: Some("10th".to_string()),
            initials: "RFH".to_string(),
            score: 175_000_000,
        },
    ]);

    Ok(pretty_assertions::assert_eq!(expected, scores))
}
