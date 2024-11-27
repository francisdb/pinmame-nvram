use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_star_wars_trilogy() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/swtril43.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("#1 Jedi".to_string()),
            short_label: Some("#1".to_string()),
            initials: "LON".to_string(),
            score: 24_000_000,
        },
        HighScore {
            label: Some("#2 Jedi".to_string()),
            short_label: Some("#2".to_string()),
            initials: "JOE".to_string(),
            score: 21_000_000,
        },
        HighScore {
            label: Some("#3 Jedi".to_string()),
            short_label: Some("#3".to_string()),
            initials: "DAY".to_string(),
            score: 19_500_000,
        },
        HighScore {
            label: Some("#4 Jedi".to_string()),
            short_label: Some("#4".to_string()),
            initials: "KRT".to_string(),
            score: 18_000_000,
        },
        HighScore {
            label: Some("#5 Jedi".to_string()),
            short_label: Some("#5".to_string()),
            initials: "MAR".to_string(),
            score: 16_500_000,
        },
        HighScore {
            label: Some("#6 Jedi".to_string()),
            short_label: Some("#6".to_string()),
            initials: "JIM".to_string(),
            score: 15_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
