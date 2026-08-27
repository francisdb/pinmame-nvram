use pinmame_nvram::{DipSwitchInfo, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

#[test]
fn test_bountyh() -> io::Result<()> {
    let nvram = Nvram::open(Path::new("testdata/bountyh.nv"))?.unwrap();

    let dips_len = nvram.dip_switches_len()?;
    assert_eq!(32, dips_len);

    let dips_info = nvram.dip_switches_info()?;
    assert_eq!(
        dips_info,
        vec![
            DipSwitchInfo {
                nr: 1,
                name: Some("Left Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 2,
                name: Some("Left Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 3,
                name: Some("Left Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 4,
                name: Some("Left Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 5,
                name: Some("Left Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 6,
                name: Some("High Games To Date".into())
            },
            DipSwitchInfo {
                nr: 7,
                name: Some("Attract Mode Sound".into())
            },
            DipSwitchInfo {
                nr: 8,
                name: Some("Auto-Percentage Control".into())
            },
            DipSwitchInfo {
                nr: 9,
                name: Some("Right Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 10,
                name: Some("Right Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 11,
                name: Some("Right Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 12,
                name: Some("Right Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 13,
                name: Some("Right Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 14,
                name: Some("Left/Right Coin Chute Control".into())
            },
            DipSwitchInfo {
                nr: 15,
                name: Some("Maximum Credits".into())
            },
            DipSwitchInfo {
                nr: 16,
                name: Some("Maximum Credits".into())
            },
            DipSwitchInfo {
                nr: 17,
                name: Some("Center Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 18,
                name: Some("Center Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 19,
                name: Some("Center Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 20,
                name: Some("Center Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 21,
                name: Some("Center Coin Chute".into())
            },
            DipSwitchInfo {
                nr: 22,
                name: Some("Playfield Special".into())
            },
            DipSwitchInfo {
                nr: 23,
                name: Some("High Score Awards".into())
            },
            DipSwitchInfo {
                nr: 24,
                name: Some("High Score Awards".into())
            },
            DipSwitchInfo {
                nr: 25,
                name: Some("Balls/Game".into())
            },
            DipSwitchInfo {
                nr: 26,
                name: Some("Match".into())
            },
            DipSwitchInfo {
                nr: 27,
                name: Some("Replay Limit".into())
            },
            DipSwitchInfo {
                nr: 28,
                name: Some("Novelty".into())
            },
            DipSwitchInfo {
                nr: 29,
                name: Some("Game Mode".into())
            },
            DipSwitchInfo {
                nr: 30,
                name: Some("3rd Coin Chute Credits".into())
            },
            DipSwitchInfo {
                nr: 31,
                name: Some("Game Setting #1".into())
            },
            DipSwitchInfo {
                nr: 32,
                name: Some("Game Setting #2".into())
            }
        ]
    );

    Ok(())
}
