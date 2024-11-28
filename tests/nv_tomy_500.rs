use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_the_whos_tommy_pinball_wizard() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/tomy_500.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    // TODO pretty sure this is wrong, is that because 500 is not compatible with 400?
    let expected = Vec::from([
        LastGamePlayer {
            score: 4_702_109_630,
            label: None,
        },
        LastGamePlayer {
            score: 4_349_414_040,
            label: None,
        },
        LastGamePlayer {
            score: 4_152_202_031,
            label: None,
        },
        LastGamePlayer {
            score: 3_720_203_230,
            label: None,
        },
    ]);
    assert_eq!(Some(expected), last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("1st".to_string()),
            short_label: Some("#1".to_string()),
            initials: "CNH".to_string(),
            score: 1_000_000_000,
        },
        HighScore {
            label: Some("2nd".to_string()),
            short_label: Some("#2".to_string()),
            initials: "IVE".to_string(),
            score: 900_000_000,
        },
        HighScore {
            label: Some("3rd".to_string()),
            short_label: Some("#3".to_string()),
            initials: "ED ".to_string(),
            score: 800_000_000,
        },
        HighScore {
            label: Some("4th".to_string()),
            short_label: Some("#4".to_string()),
            initials: "DAN".to_string(),
            score: 700_000_000,
        },
        HighScore {
            label: Some("5th".to_string()),
            short_label: Some("#5".to_string()),
            initials: "MCS".to_string(),
            score: 600_000_000,
        },
        HighScore {
            label: Some("6th".to_string()),
            short_label: Some("#6".to_string()),
            initials: "JEN".to_string(),
            score: 500_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
