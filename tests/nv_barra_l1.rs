use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_barracora() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/barra_l1.nv"))?.unwrap();

    let game_state = nvram.read_game_state()?;
    assert_eq!("29", game_state.unwrap().get("credits").unwrap());

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
