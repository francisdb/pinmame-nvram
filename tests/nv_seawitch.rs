use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;
use testdir::testdir;

#[test]
fn test_seawitch_default() -> io::Result<()> {
    let test_dir = testdir!();
    let nvram_path = test_dir.join("seawitch.nv");
    std::fs::copy("testdata/seawitch_default.nv", &nvram_path)?;
    let mut nvram = Nvram::open(&nvram_path)?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([HighScore {
        label: Some("1st".to_string()),
        short_label: Some("#1".to_string()),
        initials: "".to_string(),
        score: 0,
    }]);

    Ok(assert_eq!(expected, scores))
}

#[test]
fn test_seawitch() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/seawitch.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([HighScore {
        label: Some("1st".to_string()),
        short_label: Some("#1".to_string()),
        initials: "".to_string(),
        score: 8170,
    }]);

    Ok(assert_eq!(expected, scores))
}
