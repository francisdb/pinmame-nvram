use pinmame_nvram::{HighScore, Nvram};
use std::io;
use std::path::Path;

#[test]
fn test_dracula() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dracula.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 440_040,
    }]);

    Ok(pretty_assertions::assert_eq!(expected, scores))
}
