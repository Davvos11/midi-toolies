use nih_plug::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

struct FakeSustain {
    params: Arc<FakeSustainParams>,
    notes: HashMap<u8, NoteEvent<()>>,
    sustain_held: bool,
}

#[derive(PartialEq, Enum)]
enum BlockCC {
    #[name = "Also send original pedal CC"]
    Pass,
    #[name = "Do not set original pedal CC"]
    Block,
}

#[derive(Params)]
struct FakeSustainParams {
    #[id = "cc"]
    pub cc: IntParam,
    #[id = "block"]
    pub block: EnumParam<BlockCC>,
}

impl Default for FakeSustain {
    fn default() -> Self {
        Self {
            params: Arc::new(FakeSustainParams::default()),
            notes: HashMap::with_capacity(128),
            sustain_held: false,
        }
    }
}

impl Default for FakeSustainParams {
    fn default() -> Self {
        Self {
            cc: IntParam::new("Pedal CC", 64, IntRange::Linear { min: 0, max: 127 }),
            block: EnumParam::new("Block original CC", BlockCC::Pass),
        }
    }
}

impl Plugin for FakeSustain {
    const NAME: &'static str = "Fake Sustain";
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
            let mut send_event = true;
            match event {
                NoteEvent::NoteOff { note, .. } => {
                    // Gather note off events when pedal is held
                    if self.sustain_held {
                        self.notes.insert(note, event);
                        send_event = false;
                    }
                }
                NoteEvent::MidiCC { cc, value, .. } => {
                    if cc == self.params.cc.value() as u8 {
                        // Send gathered note off events when pedal is released
                        let sustain_held_new = value > 0.5;
                        if self.sustain_held && !sustain_held_new {
                            for (_, off_event) in self.notes.drain() {
                                context.send_event(off_event)
                            }
                        }
                        // Update held state
                        self.sustain_held = sustain_held_new;
                        // Do not send pedal event if configured to do so
                        if self.params.block.value() == BlockCC::Block {
                            send_event = false;
                        }
                    }
                }
                _ => {}
            }
            if send_event {
                context.send_event(event);
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for FakeSustain {
    const CLAP_ID: &'static str = "nl.dovatvis.fake-sustain";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Hold note off messages when sustain pedals is pressed");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for FakeSustain {
    const VST3_CLASS_ID: [u8; 16] = *b"FakeSustainnnnnn";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(FakeSustain);
nih_export_vst3!(FakeSustain);
