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

This library makes use of content from the
[Pinball Memory Maps](https://github.com/tomlogic/pinball-memory-maps) project.
The maps are embedded in the library and are used to look up values in the nvram files.

## License

The code in this repository is licensed under the [MIT license](LICENSE).

The embedded map data from the Pinball Memory Maps project is made available
under the [Open Database License (ODbL)](https://opendatacommons.org/licenses/odbl/1.0/),
with the individual contents licensed under the
[Database Contents License (DbCL)](https://opendatacommons.org/licenses/dbcl/1.0/).
Both license texts are included in the `pinball-memory-maps` submodule and ship
with the published crate.

If your application publicly displays data decoded through these maps, the ODbL
asks that you include a notice such as: "Contains information from
[Pinball Memory Maps](https://github.com/tomlogic/pinball-memory-maps), which is
made available under the
[Open Database License (ODbL)](https://opendatacommons.org/licenses/odbl/1.0/)."

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
