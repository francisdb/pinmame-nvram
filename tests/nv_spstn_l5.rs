use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_space_station() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/spstn_l5.nv"))?.unwrap();

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("scores.0".into(), "4291770".into()),
        ("scores.1".into(), "0".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
        ("credits".into(), "7".into()),
        ("current_ball".into(), "3".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    // replay 1_900_000
    //
    // default high scores
    // 1. 4_000_000 CLF
    // 2. 3_500_000 EJB
    // 3. 3_000_000 BLS
    // 4. 2_500_000 TJE
    //
    // rom high scores (last game top game using ball control)
    // (game crased before I could enter my initials)
    // 1. 4_291_770 "   "
    // 2. 4_000_000 CLF
    // 3. 3_500_000 EJB
    // 4. 3_000_000 BLS

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 4_291_770,
            label: Some("Player 1".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 2".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 3".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 4".to_string()),
        },
    ]);
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "   ".to_string(),
            score: 4_291_770,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "CLF".to_string(),
            score: 4_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "EJB".to_string(),
            score: 3_500_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BLS".to_string(),
            score: 3_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
