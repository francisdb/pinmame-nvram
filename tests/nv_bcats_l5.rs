use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;

#[test]
fn test_bad_cats() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/bcats_l5.nv"))?.unwrap();

    // replay at 3_000_000
    // jackpot worth 20_080_000

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([("credits".into(), "6".into())]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 1520990,
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
        LastGamePlayer {
            score: 0,
            label: None,
        },
    ]);
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Baddest Cat 1".to_string()),
            short_label: Some("1st".to_string()),
            initials: "SOM".to_string(),
            score: 8_697_720,
        },
        HighScore {
            label: Some("Baddest Cat 2".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "BSO".to_string(),
            score: 6_000_000,
        },
        HighScore {
            label: Some("Baddest Cat 3".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "PVA".to_string(),
            score: 5_500_000,
        },
        HighScore {
            label: Some("Baddest Cat 4".to_string()),
            short_label: Some("4th".to_string()),
            initials: "BAD".to_string(),
            score: 5_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
