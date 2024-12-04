# pinmame-nvram

Rust library handling PinMAME NVRAM files.

## Usage

```rust
use pinmame_nvram::Nvram;

fn main() {
    let mut nvram = Nvram::open(Path::new("afm_113b.nv")).unwrap().unwrap();
    let scores = nvram.read_highscores().unwrap();

    for score in &scores {
        println!("{} {} {}", score.label.unwrap(), score.initials, score.score);
    }
}
```

## Attributions

This program makes use of content from the [PinMAME NVRAM Maps](https://github.com/tomlogic/pinmame-nvram-maps) project.
The maps are embedded in the library and are used to look up values in the nvram files.
