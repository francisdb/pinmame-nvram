use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_sorcerer() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/sorcr_l1.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    assert_eq!(None, last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: None,
            short_label: None,
            initials: "".to_string(),
            score: 2_500_000,
        },
        HighScore {
            label: None,
            short_label: None,
            initials: "".to_string(),
            score: 2_000_000,
        },
        HighScore {
            label: None,
            short_label: None,
            initials: "".to_string(),
            score: 1_500_000,
        },
        HighScore {
            label: None,
            short_label: None,
            initials: "".to_string(),
            score: 1_100_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
