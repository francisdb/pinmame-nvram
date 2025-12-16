# pinmame-nvram

Rust library handling PinMAME NVRAM files.

## Usage

Add the [pinmame-nvram](https://crates.io/crates/pinmame-nvram) dependency to your project

```
cargo add pinmame-nvram
```

Example code for reading scores:

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

This library makes use of the [PinMAME NVRAM Maps](https://github.com/tomlogic/pinmame-nvram-maps) project.
The maps are embedded in the library and are used to look up values in the nvram files.

## Development

Make sure you have Rust and Cargo installed. Then clone the repository.

Check out the submodules:

```
git submodule update --init --recursive
```

Run the tests:

```
cargo test
```
