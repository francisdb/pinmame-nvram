use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_the_lost_world_jurassic_park() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/jplstw22.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("#1".to_string()),
            short_label: Some("#1".to_string()),
            initials: "JRB".to_string(),
            score: 2_400_000_000,
        },
        HighScore {
            label: Some("#2".to_string()),
            short_label: Some("#2".to_string()),
            initials: "JEK".to_string(),
            score: 2_100_000_000,
        },
        HighScore {
            label: Some("#3".to_string()),
            short_label: Some("#3".to_string()),
            initials: "NF ".to_string(),
            score: 1_950_000_000,
        },
        HighScore {
            label: Some("#4".to_string()),
            short_label: Some("#4".to_string()),
            initials: "DAY".to_string(),
            score: 1_800_000_000,
        },
        HighScore {
            label: Some("#5".to_string()),
            short_label: Some("#5".to_string()),
            initials: "KRT".to_string(),
            score: 1_650_000_000,
        },
        HighScore {
            label: Some("#6".to_string()),
            short_label: Some("#6".to_string()),
            initials: "JIM".to_string(),
            score: 1_500_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
