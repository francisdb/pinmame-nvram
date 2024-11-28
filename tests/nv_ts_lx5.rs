use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
#[ignore = "see https://github.com/tomlogic/pinmame-nvram-maps/issues/27"]
fn test_the_shadow_lx5() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/ts_lx5.nv"))?.unwrap();

    let champions = nvram.read_mode_champions()?;
    let expected = Vec::from([ModeChampion {
        label: Some("Shadow Loop Champ".to_string()),
        short_label: Some("SLC".to_string()),
        initials: "TEX".to_string(),
        score: Some(2),
        suffix: None, // TODO should be " loops"
        timestamp: None,
    }]);
    Ok(assert_eq!(Some(expected), champions))
}

#[test]
fn test_the_shadow() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/ts_lx5.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 142239830,
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
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "MAB".to_string(),
            score: 700_000_000,
        },
        HighScore {
            label: Some("Best Agent #1".to_string()),
            short_label: Some("#1".to_string()),
            initials: "BRE".to_string(),
            score: 650_000_000,
        },
        HighScore {
            label: Some("Best Agent #2".to_string()),
            short_label: Some("#2".to_string()),
            initials: "BCF".to_string(),
            score: 600_000_000,
        },
        HighScore {
            label: Some("Best Agent #3".to_string()),
            short_label: Some("#3".to_string()),
            initials: "DTW".to_string(),
            score: 550_000_000,
        },
        HighScore {
            label: Some("Best Agent #4".to_string()),
            short_label: Some("#4".to_string()),
            initials: "DWF".to_string(),
            score: 500_000_000,
        },
        HighScore {
            label: Some("Buyin Champion".to_string()),
            short_label: Some("BIC".to_string()),
            initials: "BRE".to_string(),
            score: 900_000_000,
        },
        HighScore {
            label: Some("BuyIn Score #1".to_string()),
            short_label: Some("BI#1".to_string()),
            initials: "MAB".to_string(),
            score: 850_000_000,
        },
        HighScore {
            label: Some("BuyIn Score #2".to_string()),
            short_label: Some("BI#2".to_string()),
            initials: "BCF".to_string(),
            score: 800_000_000,
        },
        HighScore {
            label: Some("BuyIn Score #3".to_string()),
            short_label: Some("BI#3".to_string()),
            initials: "TEX".to_string(),
            score: 750_000_000,
        },
        HighScore {
            label: Some("BuyIn Score #4".to_string()),
            short_label: Some("BI#4".to_string()),
            initials: "ASR".to_string(),
            score: 700_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}
