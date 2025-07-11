use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;
use testdir::testdir;

#[test]
fn test_warlok() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/wrlok_l3.nv"))?.unwrap();

    let game_state = nvram.read_game_state()?.unwrap();
    assert_eq!("30", game_state.get("credits").unwrap());
    assert_eq!("3", game_state.get("ball_count").unwrap());

    let scores = nvram.read_highscores()?;
    let expected = vec![HighScore {
        label: Some("First Place".to_string()),
        short_label: Some("1st".to_string()),
        initials: "".into(),
        score: 2_500_000,
    }];
    assert_eq!(expected, scores);

    Ok(())
}

#[test]
fn test_warlok_default_0_credits() -> io::Result<()> {
    let test_dir = testdir!();
    let nvram_path = test_dir.join("wrlok_l3.nv");
    std::fs::copy("testdata/wrlok_l3-default.nv", &nvram_path)?;
    let mut nvram = Nvram::open(&nvram_path)?.unwrap();

    let game_state = nvram.read_game_state()?.unwrap();
    assert_eq!("0", game_state.get("credits").unwrap());
    assert_eq!("3", game_state.get("ball_count").unwrap());

    Ok(())
}
