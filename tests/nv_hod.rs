use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_house_of_diamonds() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/hod.nv"))?.unwrap();

    // 6 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([("credits".into(), "8".into())]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 57_290,
            label: None,
        },
        LastGamePlayer {
            score: 10_000,
            label: None,
        },
        LastGamePlayer {
            score: 0,
            label: None,
        },
        LastGamePlayer {
            score: 0,
            label: None,
        },
    ]);
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 57_290,
    }]);
    assert_eq!(expected, scores);

    Ok(())
}
