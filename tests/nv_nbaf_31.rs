use pinmame_nvram::{HighScore, LastGamePlayer, ModeChampion, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_nba_fastbreak() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/nbaf_31.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 4,
            label: Some("Player 1".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 2".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 3".into()),
        },
        LastGamePlayer {
            score: 0,
            label: Some("Player 4".into()),
        },
    ];
    assert_eq!(Some(expected), last_game);

    let champions = nvram.read_mode_champions()?;
    let expected = vec![
        ModeChampion {
            label: Some("BLAZERS".to_string()),
            short_label: None,
            initials: Some("TMK".to_string()),
            score: Some(51),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("BUCKS".into()),
            short_label: None,
            initials: Some("CJS".into()),
            score: Some(48),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("BULLETS".into()),
            short_label: None,
            initials: Some("ZAB".into()),
            score: Some(34),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("BULLS".into()),
            short_label: None,
            initials: Some("LED".into()),
            score: Some(30136),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("CAVALIERS".into()),
            short_label: None,
            initials: Some("ROG".into()),
            score: Some(10084),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("CELTICS".into()),
            short_label: None,
            initials: Some("LFS".into()),
            score: Some(20104),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("CLIPPERS".into()),
            short_label: None,
            initials: Some("ASR".into()),
            score: Some(52),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("GRIZZLIES".into()),
            short_label: None,
            initials: Some("POP".into()),
            score: Some(41),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("HAWKS".into()),
            short_label: None,
            initials: Some("JAP".into()),
            score: Some(53),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("HEAT".into()),
            short_label: None,
            initials: Some("VJP".into()),
            score: Some(43),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("HORNETS".into()),
            short_label: None,
            initials: Some("BRE".into()),
            score: Some(47),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("JAZZ".into()),
            short_label: None,
            initials: Some("DJW".into()),
            score: Some(46),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("KINGS".into()),
            short_label: None,
            initials: Some("EAE".into()),
            score: Some(49),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("KNICKS".into()),
            short_label: None,
            initials: Some("RRR".into()),
            score: Some(50),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("LAKERS".into()),
            short_label: None,
            initials: Some("ADG".into()),
            score: Some(10072),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("MAGIC".into()),
            short_label: None,
            initials: Some("BCM".into()),
            score: Some(45),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("MAVERICKS".into()),
            short_label: None,
            initials: Some("PML".into()),
            score: Some(39),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("NETS".into()),
            short_label: None,
            initials: Some("TEX".into()),
            score: Some(58),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("NUGGETS".into()),
            short_label: None,
            initials: Some("MW ".into()),
            score: Some(35),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("PACERS".into()),
            short_label: None,
            initials: Some("TWU".into()),
            score: Some(32),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("PISTONS".into()),
            short_label: None,
            initials: Some("XAQ".into()),
            score: Some(10070),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("RAPTORS".into()),
            short_label: None,
            initials: Some("UTB".into()),
            score: Some(33),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("ROCKETS".into()),
            short_label: None,
            initials: Some("GIW".into()),
            score: Some(31),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("76ERS".into()),
            short_label: None,
            initials: Some("ZAP".into()),
            score: Some(55),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("SUPERSONICS".into()),
            short_label: None,
            initials: Some("GG ".into()),
            score: Some(54),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("SPURS".into()),
            short_label: None,
            initials: Some("KCQ".into()),
            score: Some(44),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("SUNS".into()),
            short_label: None,
            initials: Some("MAT".into()),
            score: Some(40),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("T-WOLVES".into()),
            short_label: None,
            initials: Some("KOZ".into()),
            score: Some(37),
            suffix: None,
            timestamp: None,
        },
        ModeChampion {
            label: Some("WARRIORS".into()),
            short_label: None,
            initials: Some("JIM".into()),
            score: Some(57),
            suffix: None,
            timestamp: None,
        },
    ];
    assert_eq!(Some(expected), champions);

    let scores = nvram.read_highscores()?;
    let expected = vec![
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "GG ".to_string(),
            score: 150,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "TMK".to_string(),
            score: 130,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "KCQ".to_string(),
            score: 110,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "CAT".to_string(),
            score: 90,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "MAS".to_string(),
            score: 70,
        },
    ];
    assert_eq!(expected, scores);

    Ok(())
}
