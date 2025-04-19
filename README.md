# MIDI Toolies

## Building

After installing [Rust](https://rustup.rs/), you the various tools like so:

```shell
# Keyboard mirror (mirror a keyboard around G#4)
cargo xtask bundle keyboard_mirror --release
# Fake sustain (hold notes when sustain pedal is pressed)
cargo xtask bundle fake_sustain --release
```
