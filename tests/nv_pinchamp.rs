use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_pinball_champ_82() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/pinchamp.nv"))?.unwrap();

    // 8 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([("credits".into(), "39".into())]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 342_010,
            label: None,
        },
        LastGamePlayer {
            score: 1_521_110,
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
        score: 13_500_000,
    }]);
    assert_eq!(expected, scores);

    Ok(())
}
