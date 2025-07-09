use pinmame_nvram::{LastGamePlayer, Nvram};
use pretty_assertions::assert_eq;
use std::io;
use std::path::Path;

// Settings
//
// 32 physical dip switches
//
// # High score functions
// 01 Extra ball / Free game score level 1 5954010
// 02 Extra ball / Free game score level 2 00
// 03 Extra ball / Free game score level 3 00
// 04 High score to date (can be set) 140000
// # Bookkeeping functions
// 05 Current Credits 00 to 40 - 00
// 06 Total Plays (Played and Free Games) 100000 to 99999 ??? - 35
// 07 Total Replays (Free Games) 10000 to 99999 - 02
// 08 Game Percentage 00 to 99999 - 06
// 09 Total times 'High Score To Date' is beat - 00 to 99999 - 01
// 10 Coins Dropped thru Coin Chute #1 10000 to 99999 - 105
// 11 Coins Dropped thru Coin Chute #2 10000 to 99999 - 05
// 12 Coins Dropped thru Coin Chute #3 10000 to 99999 - 50
// 13 Number of Specials awarded from Panel Specials Only 00 to 99999 - 00
// 14 Number of minutes of Game Play 00 to 99999 - 04
// 15 Number of Service Credits 00 to 99999 - 00
// # Self test functions
// 16 - 00 (00-03) - High Score award #1 - (00 No Award, 01 Novelty, 02 Extra Ball, 03 Replay)
// 17 - 00 (00-03) - High Score award #2 - (00 No Award, 01 No Award, 02 Extra Ball, 03 Replay)
// 18 - 00 (00-03) ???
// 19 - 00 (00-03) - High Score to date or over 10.000.000 score feature - (00 No Award, 01 One Credit, 02 Two Credits, 03 Three Credits)
// 20 - 15 (00-15) ???
// 21 - 15 (00-15) ???
// 22 - ?? (00-03) does not work or show anything?

#[test]
fn test_centaur() -> io::Result<()> {
    let path = Path::new("testdata/centaur.nv");
    let mut nvram = Nvram::open(path)?.unwrap();

    let last_game = nvram.read_last_game()?;
    let expected = vec![
        LastGamePlayer {
            score: 52_500,
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
    ];
    assert_eq!(Some(expected), last_game);

    // C. MEMORY BONUS FEATURE - #6 #7
    // D. POWER ORB FEATURE(MULTI-BALL) - #15 #22 #24 #21

    // #1-#5 Coin Shute #1 adjustments
    // #6 Multiplier Control - ON Multiplier in Memory / OFF Multipliers reset
    // #7 Bonus Control - ON All achieved bonus is held in memory. / OFF Only 20, 40 and 60 are held in memory.
    // #8 Sequence Feature - ON Liberal / OFF Conservative
    // #9-#13 Coin Chute #3 adjustments
    // #14 In-line drop target feature lite adjustment - ON Any feature lite ON will come ON for next ball. / OFF Feature lite ON will reset to 10 for next ball.
    // #15 Memory on Captive Orbs - ON Lit orbs remain lit until their release. / OFF Lit orbs are reset at the end of a ball.
    // #16 Guardian Feature - ON Guardians release during the entire ball / OFF Guardians can only release one ball
    // #17-#20 Coin Shute #2 adjustments
    // #21 Release on Last Ball - ON Liberal / OFF Conservative
    // #22 Captive Orb Initialization - ON Each ball begins with at least one orb lit. / OFF The player must earn all lit orbs.
    // #23 Tilt Feature - ON Ball Tilt / OFF Game Tilt
    // #24 Memory on Release - ON The lit release target remains lit. / OFF Target must be qualified on each ball.
    // #25-#26 Maximum credits adjustments
    //   40 ON  ON
    //   25 OFF ON
    //   15 ON  OFF
    //   10 OFF OFF
    // #27 Credits Displayed - ON Yes / OFF No
    // #28 Match Feature - ON Match ON / OFF Match OFF
    // #29 Number of games replays per game adjustmen - ON Liberal. All relays earned will be collected. / OFF Conservative. Only onne replay per player per game.
    // #30 Game Over Animation - ON Five balls will kick to playfield every 15 minutes / OFF No kickout of balls.
    // #32 #31 Balls per game adjustments
    //   5 OFF ON
    //   4 ON  OFF
    //   3 OFF OFF
    //   2 ON  ON

    // MDRV_DIPS(35)

    // PORT_START /* 1 */ \
    // COREPORT_DIPNAME( 0x0001, 0x0000, "S1") \
    // COREPORT_DIPSET(0x0000, "0" ) \
    // COREPORT_DIPSET(0x0001, "1" ) \
    // COREPORT_DIPNAME( 0x0002, 0x0000, "S2") \
    // COREPORT_DIPSET(0x0000, "0" ) \
    // COREPORT_DIPSET(0x0002, "1" ) \
    // COREPORT_DIPNAME( 0x0004, 0x0000, "S3") \
    // COREPORT_DIPSET(0x0000, "0" ) \
    // COREPORT_DIPSET(0x0004, "1" ) \
    // COREPORT_DIPNAME( 0x0008, 0x0000, "S4") \
    // COREPORT_DIPSET(0x0000, "0" ) \
    // COREPORT_DIPSET(0x0008, "1" ) \
    // ...
    //  PORT_START /* 2 */
    // ...
    // PORT_START /* 3 */
    // COREPORT_DIPNAME( 0x0007, 0x0004, "Reverb Effect") \
    // COREPORT_DIPSET(0x0000, "0" ) \
    // COREPORT_DIPSET(0x0001, "1" ) \
    // COREPORT_DIPSET(0x0002, "2" ) \
    // COREPORT_DIPSET(0x0003, "3" ) \
    // COREPORT_DIPSET(0x0004, "4" ) \
    // COREPORT_DIPSET(0x0005, "5" ) \
    // COREPORT_DIPSET(0x0006, "6" ) \
    // COREPORT_DIPSET(0x0007, "7" ) \

    // #define COREPORT_DIPNAME(mask,default,name) \
    //    PORT_DIPNAME(mask,default,name)
    // #define COREPORT_DIPSET(mask,name) \
    //    PORT_DIPSETTING(mask,name)

    // // set all dip switches to off
    // for switch_number in 1..=32 {
    //     nvram.set_dip_switch(switch_number, false)?;
    // }
    //
    // // enable credits displayed 27
    // nvram.set_dip_switch(27, true)?;
    //
    // // 5 balls per game 32 31
    // nvram.set_dip_switch(32, false)?;
    // nvram.set_dip_switch(31, true)?;
    //
    // // reverb effect value up to 7 (3 bits)
    // // 7 = all 3 switches on
    // nvram.set_dip_switch(33, true)?;
    // nvram.set_dip_switch(34, true)?;
    // nvram.set_dip_switch(35, true)?;

    let mut dip_switch_string = "".to_string();
    for switch_number in 1..=nvram.dip_switches_len() {
        let enabled = nvram.get_dip_switch(switch_number)?;
        let state = if enabled { "ON" } else { "OFF" };
        println!("#{} {}", switch_number, state);
        dip_switch_string.push_str(&format!("#{} {}\n", switch_number, state));
    }

    // make a string with all dip switches and compare it with the expected
    let expected = "\
#1 OFF\n\
#2 OFF\n\
#3 OFF\n\
#4 OFF\n\
#5 OFF\n\
#6 OFF\n\
#7 OFF\n\
#8 OFF\n\
#9 OFF\n\
#10 OFF\n\
#11 OFF\n\
#12 OFF\n\
#13 OFF\n\
#14 OFF\n\
#15 OFF\n\
#16 OFF\n\
#17 OFF\n\
#18 OFF\n\
#19 OFF\n\
#20 OFF\n\
#21 OFF\n\
#22 OFF\n\
#23 OFF\n\
#24 OFF\n\
#25 OFF\n\
#26 OFF\n\
#27 ON\n\
#28 OFF\n\
#29 OFF\n\
#30 OFF\n\
#31 ON\n\
#32 OFF\n\
#33 ON\n\
#34 ON\n\
#35 ON\n\
"
    .to_string();

    assert_eq!(expected, dip_switch_string);

    Ok(())
}
