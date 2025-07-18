use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_solar_fire() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/solar_l2.nv"))?.unwrap();

    let game_state = nvram.read_game_state()?.unwrap();
    assert_eq!("32", game_state.get("credits").unwrap());
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
