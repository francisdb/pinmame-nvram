use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;
use testdir::testdir;

// TODO add adjustments
//     "default_high_score": {
//       "_note": "Should probably be a adjustment, this is the default high score when pressing the highscore reset button (5 in vpinball). Default is 200",
//       "label": "High Score",
//       "start": "0x781",
//       "length": 2,
//       "encoding": "bcd"
//     }

#[test]
fn test_gold_mine() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/gmine_l2.nv"))?.unwrap();

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("credits".into(), "22".into()),
        //("default_high_score".into(), "11".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
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
    let expected = Vec::from([HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".into(),
        score: 104,
    }]);
    assert_eq!(expected, scores);

    Ok(())
}

#[test]
fn test_gold_mine_6players() -> io::Result<()> {
    let test_dir = testdir!();
    let nvram_path = test_dir.join("gmine_l2.nv");
    std::fs::copy("testdata/gmine_l2_6players.nv", &nvram_path)?;

    let mut nvram = Nvram::open(&nvram_path)?.unwrap();

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("credits".into(), "26".into()),
        //("default_high_score".into(), "200".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    // score table
    // 2237 1744
    // 1053  454
    // 1738  368

    // all 6 high scores 600

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 2237,
            label: None,
        },
        LastGamePlayer {
            score: 1744,
            label: None,
        },
        LastGamePlayer {
            score: 1053,
            label: None,
        },
        LastGamePlayer {
            score: 454,
            label: None,
        },
        LastGamePlayer {
            score: 1738,
            label: None,
        },
        LastGamePlayer {
            score: 368,
            label: None,
        },
    ]);
    assert_eq!(Some(expected), last_game);
    Ok(())
}
