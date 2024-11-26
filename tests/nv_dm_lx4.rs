use pinmame_nvram::{ChecksumMismatch, HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;
use testdir::testdir;

#[test]
fn test_demolition_man() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/dm_lx4.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "TED".to_string(),
            score: 1_250_000_000,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "WAG".to_string(),
            score: 950_000_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "DEN".to_string(),
            score: 800_000_000,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "DTW".to_string(),
            score: 650_000_000,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "HEY".to_string(),
            score: 500_000_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}

#[test]
fn test_demolition_man_clear_scores() -> io::Result<()> {
    let dir = testdir!();
    let test_file = dir.join("dm_lx4.nv");
    // copy the test file to the test directory
    std::fs::copy("testdata/dm_lx4.nv", &test_file)?;
    let mut nvram = Nvram::open(&test_file)?.unwrap();
    nvram.clear_highscores()?;
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Grand Champion".to_string()),
            short_label: Some("GC".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
    ]);

    assert_eq!(expected, scores);

    let checksum_failures = nvram.verify_all_checksum16()?;
    Ok(assert_eq!(
        Vec::<ChecksumMismatch<u16>>::new(),
        checksum_failures
    ))
}
