# MIDI Toolies

## Download

Download the latest version from the [releases page](https://github.com/Davvos11/midi-toolies/releases).

This includes the following tools as VST3 and CLAP plugins:
- Keyboard mirror (mirror a keyboard around G#4)
- Fake sustain (hold notes when sustain pedal is pressed)

## Building

After installing [Rust](https://rustup.rs/), you can build the various tools like so:

```shell
# Keyboard mirror (mirror a keyboard around G#4)
cargo xtask bundle keyboard_mirror --release
# Fake sustain (hold notes when sustain pedal is pressed)
cargo xtask bundle fake_sustain --release
```
