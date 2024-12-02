use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;
use testdir::testdir;

#[test]
fn test_white_water() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/ww_l5.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 1_680_030,
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

    let champions = nvram.read_mode_champions()?;
    let expected = Vec::from([ModeChampion {
        label: Some("Insanity Record".to_string()),
        short_label: Some("InsanityRecord".to_string()),
        initials: "RSM".to_string(),
        score: Some(12),
        suffix: Some(" Water Falls".to_string()),
        timestamp: None,
    }]);
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("RIVER MASTER".to_string()),
            short_label: Some("GC".to_string()),
            initials: "MAB".to_string(),
            score: 200_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "DEN".to_string(),
            score: 120_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "CG ".to_string(),
            score: 100_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "TEX".to_string(),
            score: 90_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "EJB".to_string(),
            score: 80_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}

#[test]
fn test_white_water_default() -> io::Result<()> {
    let testdir = testdir!();
    let rom_path = testdir.join("ww_lh6.nv");
    std::fs::copy("testdata/ww_lh6_default.nv", &rom_path)?;

    let mut nvram = Nvram::open(&rom_path)?.unwrap();

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
    ]);
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("RIVER MASTER".to_string()),
            short_label: Some("GC".to_string()),
            initials: "MAB".to_string(),
            score: 200_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "DEN".to_string(),
            score: 120_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "CG ".to_string(),
            score: 100_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "TEX".to_string(),
            score: 90_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "EJB".to_string(),
            score: 80_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
