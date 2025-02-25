use pinmame_nvram::{HighScore, LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_whirlwind() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/whirl_l3.nv"))?.unwrap();

    let replay_score = nvram.read_replay_score()?;
    assert_eq!(None, replay_score);

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
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
    ]);
    assert_eq!(Some(expected), last_game);

    let champions = nvram.read_mode_champions()?;
    assert_eq!(None, champions);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Champion".to_string()),
            short_label: Some("Champ".to_string()),
            initials: "HHR".to_string(),
            score: 10_000_000,
        },
        HighScore {
            label: Some("1st".to_string()),
            short_label: Some("#1".to_string()),
            initials: "JCY".to_string(),
            score: 6_000_000,
        },
        HighScore {
            label: Some("2nd".to_string()),
            short_label: Some("#2".to_string()),
            initials: "JRK".to_string(),
            score: 5_500_000,
        },
        HighScore {
            label: Some("3rd".to_string()),
            short_label: Some("#3".to_string()),
            initials: "CPG".to_string(),
            score: 5_000_000,
        },
        HighScore {
            label: Some("4th".to_string()),
            short_label: Some("#4".to_string()),
            initials: "PFZ".to_string(),
            score: 4_500_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
