use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;
use testdir::testdir;

#[test]
fn test_seawitch_default() -> io::Result<()> {
    let test_dir = testdir!();
    let nvram_path = test_dir.join("seawitch.nv");
    std::fs::copy("testdata/seawitch-default.nv", &nvram_path)?;
    let mut nvram = Nvram::open(&nvram_path)?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = vec![HighScore {
        label: Some("1st".to_string()),
        short_label: Some("#1".to_string()),
        initials: "".to_string(),
        score: 0,
    }];
    assert_eq!(expected, scores);

    Ok(())
}

#[test]
fn test_seawitch() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/seawitch.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    assert_eq!(None, last_game);

    let scores = nvram.read_highscores()?;
    let expected = vec![HighScore {
        label: Some("1st".to_string()),
        short_label: Some("#1".to_string()),
        initials: "".to_string(),
        score: 8170,
    }];
    assert_eq!(expected, scores);

    Ok(())
}
