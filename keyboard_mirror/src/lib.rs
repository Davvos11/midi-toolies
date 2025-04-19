use nih_plug::prelude::*;
use std::sync::Arc;

struct KeyboardMirror {
    params: Arc<KeyboardMirrorParams>,
}

#[derive(Params, Default)]
struct KeyboardMirrorParams {}

impl Default for KeyboardMirror {
    fn default() -> Self {
        Self {
            params: Arc::new(KeyboardMirrorParams::default()),
        }
    }
}

const MIRROR: i8 = 68; // G#4

impl Plugin for KeyboardMirror {
    const NAME: &'static str = "Keyboard Mirror";
    const VENDOR: &'static str = "Davvos11";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "vosdavid2@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];
    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { timing, voice_id, channel, note, velocity }  => {
                    let mirrored = mirror_note(note);
                    let event = NoteEvent::NoteOn { timing, voice_id, channel, note: mirrored, velocity };
                    context.send_event(event);
                }
                NoteEvent::NoteOff { timing, voice_id, channel, note, velocity } => {
                    let mirrored = mirror_note(note);
                    let event = NoteEvent::NoteOff { timing, voice_id, channel, note: mirrored, velocity };
                    context.send_event(event);
                }
                _ => {
                    context.send_event(event);
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for KeyboardMirror {
    const CLAP_ID: &'static str = "nl.dovatvis.keyboard-mirror";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Mirrors a MIDI keyboard ofzo");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for KeyboardMirror {
    const VST3_CLASS_ID: [u8; 16] = *b"KeyboardMirrorrr";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

fn mirror_note(note: u8) -> u8 {
    let distance = note as i8 - MIRROR;
    (MIRROR - distance) as u8
}

nih_export_clap!(KeyboardMirror);
nih_export_vst3!(KeyboardMirror);
