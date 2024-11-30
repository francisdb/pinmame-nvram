use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_laser_cue() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/lsrcu_l2.nv"))?.unwrap();

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([("credits".into(), "31".into())]);
    assert_eq!(Some(expected), game_state);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "".into(),
            score: 2_500_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "".into(),
            score: 2_100_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "".into(),
            score: 1_700_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "".into(),
            score: 1_300_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
