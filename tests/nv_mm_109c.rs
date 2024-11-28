use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_medieval_madness() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/mm_109c.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = Vec::from([
        LastGamePlayer {
            score: 587_880,
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
    let expected = Vec::from([
        ModeChampion {
            label: Some("Castle Champion".into()),
            short_label: Some("Castle Champ".into()),
            initials: "JCY".into(),
            score: Some(6),
            suffix: Some(" Castles Destroyed".into()),
            timestamp: None,
        },
        ModeChampion {
            label: Some("Joust Champion".into()),
            short_label: Some("Joust Champ".into()),
            initials: "DWF".into(),
            score: Some(5),
            suffix: Some(" Joust Victories".into()),
            timestamp: None,
        },
        ModeChampion {
            label: Some("Catapult Champion".into()),
            short_label: Some("Catapult Champ".into()),
            initials: "ASR".into(),
            score: Some(5),
            suffix: Some(" Catapult Slams".into()),
            timestamp: None,
        },
        ModeChampion {
            label: Some("Peasant Champion".into()),
            short_label: Some("Peasant Champ".into()),
            initials: "BCM".into(),
            score: Some(5),
            suffix: Some(" Peasant Revolts".into()),
            timestamp: None,
        },
        ModeChampion {
            label: Some("Damsel Champion".into()),
            short_label: Some("Damsel Champ".into()),
            initials: "DJW".into(),
            score: Some(5),
            suffix: Some(" Damsels Saved".into()),
            timestamp: None,
        },
        ModeChampion {
            label: Some("Troll Champion".into()),
            short_label: Some("Troll Champ".into()),
            initials: "JCD".into(),
            score: Some(20),
            suffix: Some(" Trolls Destroyed".into()),
            timestamp: None,
        },
        ModeChampion {
            label: Some("Madness Champion".into()),
            short_label: Some("Madness Champ".into()),
            initials: "KOZ".into(),
            score: Some(20000000),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("King of the Realm #1".into()),
            short_label: Some("King #1".into()),
            initials: "KOP".into(),
            score: None,
            suffix: None,
            timestamp: Some("2024-01-17 14:48".into()),
        },
        ModeChampion {
            label: Some("King of the Realm #2".into()),
            short_label: Some("King #2".into()),
            initials: "KOP".into(),
            score: None,
            suffix: None,
            timestamp: Some("2024-01-17 14:48".into()),
        },
        ModeChampion {
            label: Some("King of the Realm #3".into()),
            short_label: Some("King #3".into()),
            initials: "KOP".into(),
            score: None,
            suffix: None,
            timestamp: Some("2024-01-17 14:48".into()),
        },
        ModeChampion {
            label: Some("King of the Realm #4".into()),
            short_label: Some("King #4".into()),
            initials: "KOP".into(),
            score: None,
            suffix: None,
            timestamp: Some("2024-01-17 14:48".into()),
        },
    ]);
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".into()),
            short_label: Some("GC".into()),
            initials: "SLL".into(),
            score: 52_000_000,
        },
        HighScore {
            label: Some("First Place".into()),
            short_label: Some("1st".into()),
            initials: "BRE".into(),
            score: 44_000_000,
        },
        HighScore {
            label: Some("Second Place".into()),
            short_label: Some("2nd".into()),
            initials: "LFS".into(),
            score: 40_000_000,
        },
        HighScore {
            label: Some("Third Place".into()),
            short_label: Some("3rd".into()),
            initials: "ZAP".into(),
            score: 36_000_000,
        },
        HighScore {
            label: Some("Fourth Place".into()),
            short_label: Some("4th".into()),
            initials: "RCF".into(),
            score: 32_000_000,
        },
    ]);
    assert_eq!(expected, scores);

    Ok(())
}
