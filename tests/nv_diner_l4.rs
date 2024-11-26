use pinmame_nvram::{ChecksumMismatch, HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;
use testdir::testdir;

#[test]
fn test_diner() -> io::Result<()> {
    let mut nvram = Nvram::open(Path::new("testdata/diner_l4.nv"))?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Highest Score #1".to_string()),
            short_label: Some("#1".to_string()),
            initials: "TTR".to_string(),
            score: 8_000_000,
        },
        HighScore {
            label: Some("Highest Score #2".to_string()),
            short_label: Some("#2".to_string()),
            initials: "RMR".to_string(),
            score: 7_500_000,
        },
        HighScore {
            label: Some("Highest Score #3".to_string()),
            short_label: Some("#3".to_string()),
            initials: "ABG".to_string(),
            score: 7_000_000,
        },
        HighScore {
            label: Some("Highest Score #4".to_string()),
            short_label: Some("#4".to_string()),
            initials: "CDG".to_string(),
            score: 6_500_000,
        },
    ]);

    Ok(assert_eq!(expected, scores))
}

#[test]
fn test_diner_clear_scores() -> io::Result<()> {
    let dir = testdir!();
    let test_file = dir.join("diner_l4.nv");
    // copy the test file to the test directory
    std::fs::copy("testdata/diner_l4.nv", &test_file)?;
    let mut nvram = Nvram::open(&test_file)?.unwrap();
    nvram.clear_highscores()?;
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("Highest Score #1".to_string()),
            short_label: Some("#1".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Highest Score #2".to_string()),
            short_label: Some("#2".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Highest Score #3".to_string()),
            short_label: Some("#3".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Highest Score #4".to_string()),
            short_label: Some("#4".to_string()),
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
