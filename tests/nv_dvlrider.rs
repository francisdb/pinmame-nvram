use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_devil_riders() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dvlrider.nv"))?.unwrap();

    // 7 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([("credits".into(), "37".into())]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 200430,
            label: None,
        },
        LastGamePlayer {
            score: 129760,
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
        score: 4_000_000,
    }]);
    assert_eq!(expected, scores);

    Ok(())
}
