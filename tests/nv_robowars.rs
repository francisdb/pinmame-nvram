use pinmame_nvram::{HighScore, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;
use testdir::testdir;

#[test]
fn test_robo_war() -> io::Result<()> {
    // TODO some strange observations:
    //   - The initials seem to be stored like this: 05 53 04 4F 04 4D
    //     which is "SOM" in ASCII, but the first byte is always repeated
    //     in the second byte as high value

    let mut nvram = Nvram::open(Path::new("testdata/robowars.nv"))?.unwrap();

    let last_game = nvram.read_last_game()?;
    assert_eq!(None, last_game);

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            // default is "\0\0\0\0\0\0\0"
            initials: "SOM".to_string(),
            // default is 0
            score: 123_000,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "AAA".to_string(),
            score: 122090,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "DDD".to_string(),
            score: 116030,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "CCC".to_string(),
            score: 88210,
        },
        HighScore {
            label: Some("Fifth Place".to_string()),
            short_label: Some("5th".to_string()),
            initials: "BBB".to_string(),
            score: 87000,
        },
    ]);

    assert_eq!(expected, scores);
    Ok(())
}

#[test]
fn test_robo_war_default() -> io::Result<()> {
    // we have see a case where the last score was not 0
    // but 18_000_000, not sure how this happened
    let test_dir = testdir!();
    let nvram_path = test_dir.join("robowars.nv");
    // copy robowars_default.nv to test_dir/robowars.nv
    std::fs::copy("testdata/robowars_default.nv", &nvram_path)?;

    let mut nvram = Nvram::open(&nvram_path)?.unwrap();
    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            // default is "\0\0\0\0\0\0\0"
            initials: "".to_string(),
            // default is 0
            score: 0,
        },
        HighScore {
            label: Some("Second Place".to_string()),
            short_label: Some("2nd".to_string()),
            initials: "".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Third Place".to_string()),
            short_label: Some("3rd".to_string()),
            initials: "".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Fourth Place".to_string()),
            short_label: Some("4th".to_string()),
            initials: "".to_string(),
            score: 0,
        },
        HighScore {
            label: Some("Fifth Place".to_string()),
            short_label: Some("5th".to_string()),
            initials: "".to_string(),
            score: 0,
        },
    ]);

    assert_eq!(expected, scores);
    Ok(())
}

#[test]
fn test_robo_wars_clear_highscores() -> io::Result<()> {
    let test_dir = testdir!();
    let nvram_path = test_dir.join("robowars.nv");
    // copy robowars_default.nv to test_dir/robowars.nv
    std::fs::copy("testdata/robowars.nv", &nvram_path)?;

    let mut nvram = Nvram::open(&nvram_path)?.unwrap();
    nvram.clear_highscores()?;

    let scores = nvram.read_highscores()?;
    let expected = Vec::from([
        HighScore {
            label: Some("First Place".to_string()),
            short_label: Some("1st".to_string()),
            // default is "\0\0\0\0\0\0\0"
            initials: "AAA".to_string(),
            // default is 0
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
        HighScore {
            label: Some("Fifth Place".to_string()),
            short_label: Some("5th".to_string()),
            initials: "AAA".to_string(),
            score: 0,
        },
    ]);

    assert_eq!(expected, scores);
    Ok(())
}
