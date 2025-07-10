use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::io;
use std::path::Path;
use testdir::testdir;

#[test]
fn test_farfalla() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/farfalla.nv"))?.unwrap();

    // 7 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("credits".into(), "48".into()),
        ("scores.0".into(), "94220".into()),
        ("scores.1".into(), "122970".into()),
        ("scores.2".into(), "166030".into()),
        ("scores.3".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 94_220,
            label: Some("Player 1".to_string()),
        },
        LastGamePlayer {
            score: 122_970,
            label: Some("Player 2".to_string()),
        },
        LastGamePlayer {
            score: 166_030,
            label: Some("Player 3".to_string()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 4".to_string()),
        },
    ];
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = vec![HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 4_000_000,
    }];
    assert_eq!(expected, scores);

    Ok(())
}

#[test]
fn test_farfalla_default_hs_adjusted() -> io::Result<()> {
    let testdir = testdir!();
    let rom_path = testdir.join("farfalla.nv");
    std::fs::copy("testdata/farfalla-hs-135.nv", &rom_path)?;

    let mut nvram = Nvram::open(&rom_path)?.unwrap();

    // 7 digits for the score

    let game_state = nvram.read_game_state()?;
    let expected = HashMap::from([
        ("credits".into(), "0".into()),
        ("scores.0".into(), "0".into()),
        ("scores.1".into(), "0".into()),
        ("scores.2".into(), "0".into()),
        ("scores.3".into(), "0".into()),
    ]);
    assert_eq!(Some(expected), game_state);

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 0,
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
    ];
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = vec![HighScore {
        label: Some("High Score".to_string()),
        short_label: Some("HS".to_string()),
        initials: "".to_string(),
        score: 1_350_000,
    }];
    assert_eq!(expected, scores);

    Ok(())
}
